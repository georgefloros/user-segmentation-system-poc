use axum_prometheus::{
    metrics_exporter_prometheus::PrometheusHandle, GenericMetricLayer, MakeDefaultHandle,
    PrometheusMetricLayer,
};
use once_cell::sync::OnceCell;

pub fn PROMETHEUS_PAIR() -> &'static (PrometheusMetricLayer<'static>, PrometheusHandle) {
    static INSTANCE: OnceCell<(PrometheusMetricLayer<'static>, PrometheusHandle)> = OnceCell::new();
    INSTANCE.get_or_init(|| PrometheusMetricLayer::pair())
}
