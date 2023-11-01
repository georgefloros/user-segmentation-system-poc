use std::{future::ready, net::SocketAddr};

use crate::routes::metrics::routes;
use axum::{routing::get, Router};

pub async fn start_metrics_server() {
    tracing::info!("starting Metrics server");
    let port: u16 = std::env::var("METRICS_PORT")
        .unwrap_or_else(|_| "4001".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Metrics server listening on {}", addr);
    let app = routes();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
