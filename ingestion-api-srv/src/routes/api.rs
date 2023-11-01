use axum::{middleware, routing::post, Router};
use axum_prometheus::PrometheusMetricLayer;

use crate::{controllers::api::post_event_handler, helpers::PROMETHEUS_PAIR};

pub fn routes() -> Router {
    let prometheus_layer = PROMETHEUS_PAIR().0.clone();
    Router::new()
        .route("/api/v1/events", post(post_event_handler))
        .route_layer(prometheus_layer)
}
