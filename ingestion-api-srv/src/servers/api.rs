use std::{net::SocketAddr, time::Duration};

use crate::routes::api::routes;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub async fn start_api_server() {
    tracing::info!("starting API server");
    let port: u16 = std::env::var("API_PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("API server listening on {}", addr);
    let app = routes();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
