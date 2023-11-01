use axum::{body::Body, response::IntoResponse, Json};
use axum_prometheus::metrics::counter;
use rdkafka::producer::FutureRecord;
use serde::Serialize;
use serde_json::json;
use std::{
    sync::{atomic::AtomicU32, Arc},
    time::Duration,
};
use tracing::event;

use crate::{
    helpers::{get_databend_connection, ERROR_PRODUCER, EVENT_INGESTED_PRODUCER},
    models::{EventsResponse, GetUserResponse, UserEvent},
    routes::metrics as routes_metrics,
};
use routes_metrics::routes;

pub async fn post_event_handler(
    axum::Json(mut user_event): axum::extract::Json<UserEvent>,
) -> impl IntoResponse {
    let error_fn = |err: mobc::Error<databend_driver::Error>, user_event: UserEvent| async move {
        tracing::error!("Error: {:?}", err);
        counter!("events.post.error", 1);
        tokio::spawn(async move {
            //produce message to error_messages topic
            //for simplicity we will produce a message without creating a struct
            let error_msg = json!({
                "error_type": "ingest_event_error",
                "data": user_event
            });
            let topic = option_env!("ERROR_TOPIC").unwrap_or("error_topic");
            match ERROR_PRODUCER()
                .send(
                    FutureRecord::to(topic)
                        .payload(&serde_json::to_string(&error_msg).unwrap())
                        .key(user_event.client_ref_id.as_str()),
                    Duration::from_secs(0),
                )
                .await
            {
                Ok(delivery) => {
                    tracing::info!("message deliver {:?}", user_event);
                    counter!("errors.produced.success", 1);
                }
                Err((e, _)) => {
                    tracing::error!("message delivery  Error: {:?}", e);
                    counter!("errors.produced.failed", 1);
                }
            };
        });

        //return error response
        let json = Json(EventsResponse {
            ok: false,
            error: Some(err.to_string()),
        });
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, json)
    };

    // // fetch the user id from the configuration api by client_ref_id and validate that user exists
    let url = format!(
        "{}/users/client-ref-id/{}",
        option_env!("CONFIGURATION_API_URL").unwrap_or("http://localhost:3000/api/v1"),
        user_event.client_ref_id
    );
    match reqwest::get(&url)
        .await
        .unwrap()
        .json::<GetUserResponse>()
        .await
    {
        Ok(response) => match response.data {
            Some(user) => {
                tracing::info!("user found {:?}", user);
                counter!("post_event_handler.users.found", 1);
                // let event = event_arc.clone();
                match get_databend_connection().await {
                    Ok(conn) => match conn.exec(&*user_event.as_insert_query(user.id)).await {
                        Ok(_) => {
                            tokio::spawn(async move {
                                tracing::info!("produce message to kafka");

                                user_event.set_user_id(user.id);
                                let topic = option_env!("EVENT_INGESTED_TOPIC")
                                    .unwrap_or("event_ingested_topic");
                                match EVENT_INGESTED_PRODUCER()
                                    .send(
                                        FutureRecord::to(topic)
                                            .payload(&serde_json::to_string(&user_event).unwrap())
                                            .key(user_event.client_ref_id.as_str()),
                                        Duration::from_secs(0),
                                    )
                                    .await
                                {
                                    Ok(delivery) => {
                                        tracing::info!("message deliver {:?}", user_event);
                                        counter!("events.produced.success", 1);
                                    }
                                    Err((e, _)) => {
                                        tracing::error!("message delivery  Error: {:?}", e);
                                        counter!("events.produced.failed", 1);
                                    }
                                };
                            });

                            //return success response
                            counter!("events.post.success", 1);
                            let json = Json(EventsResponse {
                                ok: true,
                                error: None,
                            });
                            (axum::http::StatusCode::OK, json)
                        }
                        Err(err) => error_fn(mobc::Error::Inner(err), user_event).await,
                    },
                    Err(err) => error_fn(err, user_event).await,
                }
            }
            None => {
                tracing::error!("user not found {:?}", user_event);
                counter!("post_event_handler.users.not_found", 1);
                return error_fn(
                    mobc::Error::Inner(databend_driver::Error::InvalidResponse(
                        "USER_NOT_FOUND".to_string(),
                    )),
                    user_event,
                )
                .await;
            }
        },
        Err(err) => {
            tracing::error!("get user error  {:?}", err);
            return error_fn(
                mobc::Error::Inner(databend_driver::Error::InvalidResponse(
                    "GET_USER_ERROR".to_string(),
                )),
                user_event,
            )
            .await;
        }
    }
}
