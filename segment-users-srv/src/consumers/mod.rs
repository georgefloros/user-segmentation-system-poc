use std::io::Error;
use std::option;
use std::time::Instant;

use futures::future::join_all;
use metrics::histogram;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::FromBytes;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, Message};
use tracing::instrument;

use crate::models::UserEvent;
use crate::services::process_user_event;

#[instrument]
async fn start_consuming_msgs() {
    let brokers = option_env!("REDPANDA_BROKERS").unwrap_or("localhost:9091");
    let consumer_group = option_env!("CONSUMER_GROUP").unwrap_or("consumer_group_name");
    let enable_auto_commit = option_env!("ENABLE_AUTO_COMMIT").unwrap_or("false");
    let offset_reset = option_env!("OFFSET_RESET").unwrap_or("earliest");
    let event_ingested_topic =
        option_env!("EVENT_INGESTED_TOPIC").unwrap_or("event_ingested_topic");

    // most important config options will be passed through environment variables
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", consumer_group)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", enable_auto_commit)
        .set("auto.commit.interval.ms", "1000")
        .set("enable.auto.offset.store", "false")
        .set("auto.offset.reset", offset_reset)
        .set_log_level(RDKafkaLogLevel::Info)
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&[&event_ingested_topic])
        .expect("Can't subscribe to specified topic");

    loop {
        match consumer.recv().await {
            Err(e) => tracing::error!("Error consuming message {:?}", e),
            Ok(msg) => {
                let payload: Option<UserEvent> = match msg.payload_view::<[u8]>() {
                    None => {
                        tracing::error!("Error empty message {:?}", msg);
                        None
                    }
                    Some(Ok(payload)) => match serde_json::from_slice(payload) {
                        Ok(userEvent) => Some(userEvent),
                        Err(e) => {
                            tracing::error!("Error while deserializing message payload: {:?} ", e);
                            None
                            // produce error message to error topic
                        }
                    },
                    Some(Err(e)) => {
                        tracing::error!("Error while deserializing message payload: {:?} ", e);
                        None
                        // produce error message to error topic
                    }
                };
                if let Some(userEvent) = payload {
                    tracing::info!("Received message: {:?}", userEvent);
                    metrics::counter!("user_event_ingested", 1);
                    let start = Instant::now();
                    match process_user_event(&userEvent).await {
                        Ok(_) => {
                            metrics::counter!("user_event_processed", 1);
                        }
                        Err(e) => {
                            tracing::error!("Error while processing message: {:?} ", e);
                            metrics::counter!("user_event_processing_error", 1);
                            // produce error message to error topic
                        }
                    };
                    let delta = start.elapsed();
                    histogram!("user_event_process_time", delta);
                }
                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
        }
    }
}
#[instrument]
pub async fn start_consumers() {
    let mut futures = vec![];
    let num_workers: i32 = option_env!("NUM_OF_CONSUMERS")
        .unwrap_or("1")
        .parse()
        .unwrap();
    for i in 0..num_workers {
        futures.push(tokio::spawn(start_consuming_msgs()));
    }
    join_all(futures).await;
}
