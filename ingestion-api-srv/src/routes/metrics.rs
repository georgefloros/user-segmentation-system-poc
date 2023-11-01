use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;

use crate::helpers::PROMETHEUS_PAIR;

pub fn routes() -> Router {
    let metric_handle = PROMETHEUS_PAIR().1.clone();
    Router::new().route("/metrics", get(|| async move { metric_handle.render() }))
}
