use std::error::Error;

use databend_driver::Error as DatabendError;
use tracing::instrument;

use crate::{
    helpers::get_databend_connection,
    models::{Segment, SegmentsResponse, UserEvent},
};

#[instrument]
pub async fn process_user_event(user_event: UserEvent) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Processing user event: {:?}", user_event);
    let url = format!(
        "{}/segments/generics-and-by/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        user_event.payload.activity_type
    );
    let segments = reqwest::get(&url).await?.json::<SegmentsResponse>().await?;
    tracing::debug!("Segments: {:?}", segments);
    let mut futures = vec![];
    for s in segments.data {
        let user_event_id = user_event.id.clone().as_str();
        futures.push(tokio::spawn(evaluate_segment(user_event_id, &s)));
    }
    Ok(())
}

async fn evaluate_segment(
    user_id: &str,
    segment: &Segment,
) -> Result<(String, String), DatabendError> {
    //run query
    if segment.where_statement.is_empty() {
        return Ok((String::from(""), String::from("")));
    }

    match get_databend_connection().await {
        Ok(mut conn) => {
            let query = format!(
                "SELECT id,user_id FROM user_segment_analytics.events WHERE user_id = {} AND {} LIMIT 1",
                user_id, segment.where_statement
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
