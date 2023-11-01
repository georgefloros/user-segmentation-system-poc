use once_cell::sync::OnceCell;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;

pub fn NOTIFY_PRODUCER() -> &'static FutureProducer {
    static INSTANCE: OnceCell<FutureProducer> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let brokers =
            std::env::var("REDPANDA_BROKERS").unwrap_or_else(|_| "localhost:9091".to_string());
        ClientConfig::new()
            .set("bootstrap.servers", brokers.as_str())
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error")
    })
}
