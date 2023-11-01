use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;


pub fn routes() -> Router {
    let metric_handle = PrometheusMetricLayer::pair().1;
    Router::new().route("/metrics", get(|| async move { metric_handle.render() }))
}
