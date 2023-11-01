use std::error::Error;

use databend_driver::Error as DatabendError;
use tracing::instrument;

use crate::{
    helpers::get_databend_connection,
    models::{Segment, SegmentsResponse, UserEvent},
};

#[instrument]
async fn get_segments(activity_type: &str) ->Result<SegmentsResponse, reqwest::Error>{
    let url = format!(
        "{}/segments/generics-and-by/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        activity_type
    );
    reqwest::get(&url).await?.json::<SegmentsResponse>().await
}
#[instrument]
async fn get_user(user_Id:&str) ->Result<SegmentsResponse, reqwest::Error>{
    let url = format!(
        "{}/users/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        user_Id
    );
    reqwest::get(&url).await?.json::<SegmentsResponse>().await
}


#[instrument]
pub async fn process_user_event(user_event: UserEvent) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Processing user event: {:?}", user_event);
    
    //get segments and user records
    match tokio::try_join!(
        tokio::spawn(),
    )
    
    
    
    let url = format!(
        "{}/segments/generics-and-by/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        user_event.payload.activity_type
    );


    let segments = 
    tracing::debug!("Segments: {:?}", segments);



    let mut futures = vec![];
    for s in segments.data {
        let user_event_id = user_event.id.clone();
        let s_where = s.where_statement.clone();
        futures.push(tokio::spawn(evaluate_segment(user_event_id, s_where)));
    }
    let results = futures::future::join_all(futures).await;
    for r in results {
        let (event_id, user_id) = r?;
        if !event_id.is_empty() {
            let segment = Segment {
                id: 0,
                title: String::from(""),
                description: String::from(""),
                tag: String::from(""),
                where_statement: String::from(""),
            };
            let url = format!(
                "{}/segments/{}/users/{}",
                option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
                segment.id,
                user_id
            );
            let _ = reqwest::post(&url).await?;
        }
    }

    Ok(())
}

async fn evaluate_segment(
    user_id: String,
    segment_where: String,
) -> Result<(String, String), DatabendError> {
    //run query
    if segment_where.is_empty() {
        return Ok((String::from(""), user_id.to_string()));
    }

    match get_databend_connection().await {
        Ok(mut conn) => {
            let query = format!(
                "SELECT id,user_id FROM user_segment_analytics.events WHERE user_id = {} AND {} LIMIT 1",
                user_id,segment_where
            );
            // Hard assumption that the query will return only one row,for the sake of POC
            let mut row = conn.query_row(query.as_str()).await?.unwrap();
            let (event_id, user_id): (String, String) = row.try_into().unwrap();
            Ok((event_id, user_id))
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            Err(DatabendError::Transport("MOBC Error".to_string()))
        }
    }
}
