use std::{collections::HashMap, error::Error, hash::Hash, sync::Arc, time::Duration};

use databend_driver::Error as DatabendError;
use metrics::counter;
use rdkafka::producer::FutureRecord;
use serde_json::json;
use tracing::instrument;

use crate::{
    helpers::{get_databend_connection, NOTIFY_PRODUCER},
    models::{GetSegmentsResponse, GetUserResponse, Segment, UserEvent, UserSegment},
};

#[instrument]
async fn get_segments(activity_type: &str) -> Result<GetSegmentsResponse, reqwest::Error> {
    let url = format!(
        "{}/segments/generics-and-by/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        activity_type
    );
    match reqwest::get(&url).await {
        Ok(resp) => {
            tracing::info!("response: {:?}", resp);
            resp.json::<GetSegmentsResponse>().await
        }
        Err(e) => {
            tracing::error!("Error while getting segments: {:?}", e);
            Err(e)
        }
    }
}
#[instrument]
async fn get_user(user_Id: u16) -> Result<GetUserResponse, reqwest::Error> {
    let url = format!(
        "{}/users/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        user_Id
    );
    match reqwest::get(&url).await {
        Ok(resp) => {
            tracing::info!("response: {:?}", resp);
            resp.json::<GetUserResponse>().await
        }
        Err(e) => {
            tracing::error!("Error while getting user: {:?}", e);
            Err(e)
        }
    }
}

#[instrument]
async fn update_user_segment(
    user_Id: u16,
    segments: Arc<Vec<u16>>,
) -> Result<GetUserResponse, reqwest::Error> {
    let url = format!(
        "{}/users/{}/segments",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        user_Id
    );
    let client = reqwest::Client::new();
    let user = client
        .put(&url)
        .json(&json!({
            "segments": *segments
        }))
        .send()
        .await?
        .json::<GetUserResponse>()
        .await?;
    Ok(user)
}
#[instrument]
async fn notify_user_update_segment(
    user_Id: u16,
    in_segments: Arc<Vec<u16>>,
    out_segments: &Vec<u16>,
    key: &str,
) -> Result<(), reqwest::Error> {
    let notf_msg = json!({
        "user_id": user_Id,
        "in_segments": *in_segments,
        "out_segments": out_segments
    });
    let topic = option_env!("NOTIFICATION_TOPIC").unwrap_or("set_segment_notification_topic");

    match (NOTIFY_PRODUCER()) 
        .send(
            FutureRecord::to(topic)
                .payload(&serde_json::to_string(&notf_msg).unwrap())
                .key(key),
            Duration::from_secs(0),
        )
        .await
    {
        Ok(delivery) => {
            tracing::info!("message deliver {:?}", notf_msg);
            counter!("user_segments_notification.produced.success", 1);
        }
        Err((e, _)) => {
            tracing::error!("message delivery  Error: {:?}", e);
            counter!("user_segments_notification.produced.failed", 1);
        }
    };

    Ok(())
}

#[instrument]
pub async fn process_user_event(
    user_event: &UserEvent,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Processing user event: {:?}", user_event);

    //get segments and user records concurrently
    let (segments, user) = tokio::join!(
        get_segments(&user_event.payload.activity_type),
        get_user(user_event.user_id)
    );
    let segments = segments?;
    let user = user?;
    tracing::debug!("User: {:?}", user);
    tracing::debug!("Segments: {:?}", segments);
    // evaluate segments concurrently
    let mut futures = vec![];
    let mut segment_map: HashMap<u16, &Segment> = HashMap::new();
    for s in segments.data.iter() {
        let user_event_id = user_event.user_id;
        let s_where = s.where_statement.clone();
        let s_id = s.id;
        //Todo: avoid cloning here
        let s_tag = s.tag.clone();
        segment_map.insert(s.id, s);
        futures.push(tokio::spawn(evaluate_segment(user_event_id, s_id, s_where)));
    }

    let results = futures::future::join_all(futures).await;
    let mut new_segment_ids = vec![];
    for r in results {
        if r.is_err() {
            tracing::error!("Error while evaluating segment: {:?}", r.err());
            metrics::counter!("segment_evaluation_error", 1);
            continue;
        }
        match r.unwrap() {
            Ok((segment_id, user_id)) => {
                if user_id != 0 {
                    //TODO: avoid cloning here
                    new_segment_ids.push(segment_id);
                }
            }
            Err(e) => {
                //TODO: remove redundant logging
                tracing::error!("Error while evaluating segment: {:?}", e);
                metrics::counter!("segment_evaluation_error", 1);
            }
        };
    }
    if new_segment_ids.is_empty() {
        tracing::info!("User is not in any segment");
        return Ok(());
    }

    let mut segment_ids_to_remove = vec![];
    let mut user_segment_map: HashMap<u16, UserSegment> = user
        .data
        .segments
        .unwrap_or(vec![])
        .into_iter()
        .map(|s| (s.segment_id, s))
        .collect();

    //check if user is in segment
    //TODO: use rayon to parallelize this
    for s in segments.data.iter() {
        if new_segment_ids.contains(&s.id) && !user_segment_map.contains_key(&s.id) {
            //add user to segment
            metrics::increment_gauge!(format!("users_in_segment_{}", s.tag), 1.0);
        } else if !new_segment_ids.contains(&s.id) && user_segment_map.contains_key(&s.id) {
            // remove user from segment
            segment_ids_to_remove.push(s.id);
            metrics::decrement_gauge!(format!("users_in_segment_{}", s.tag), 1.0);
        }
    }
    //
    let new_segment_ids_arc = Arc::new(new_segment_ids);
    let (user, ()) = tokio::try_join!(
        update_user_segment(user_event.user_id, new_segment_ids_arc.clone()),
        notify_user_update_segment(
            user_event.user_id,
            new_segment_ids_arc.clone(),
            &segment_ids_to_remove,
            &user_event.client_ref_id.as_str()
        )
    )
    .unwrap();
    //send notification to kafka
    Ok(())
}

// Returns segment_id, user_id if user is in segment
#[instrument]
async fn evaluate_segment(
    user_id: u16,
    segment_id: u16,
    segment_where: String,
) -> Result<(u16, u16), DatabendError> {
    //run query
    if segment_where.is_empty() {
        return Ok((segment_id, user_id));
    }
    match get_databend_connection().await {
        Ok(mut conn) => {
            let query = format!(
                "SELECT user_id FROM user_segment_analytics.events WHERE user_id = {} AND {} LIMIT 1",
                user_id,segment_where
            );
            // Hard assumption that the query will return only one row,for the sake of POC
            let mut row = conn.query_row(query.as_str()).await?.unwrap();
            //TODO: find a workaround to this. This is a hack to get the user_id from the row
            let user_id: String = match row.is_empty() {
                true => "0".to_string(),
                false => row.values().get(0).unwrap().to_string(),
            };
            Ok((segment_id, user_id.parse().unwrap()))
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            Err(DatabendError::Transport("MOBC Error".to_string()))
        }
    }
}
