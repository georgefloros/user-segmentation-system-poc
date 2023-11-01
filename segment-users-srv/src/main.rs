#![recursion_limit = "1024"]
#![allow(warnings)] // for dev

#[path = "servers/mod.rs"]
pub mod servers;

#[path = "models/mod.rs"]
pub mod models;

#[path = "routes/mod.rs"]
pub mod routes;

#[path = "helpers/mod.rs"]
pub mod helpers;

#[path = "consumers/mod.rs"]
pub mod consumers;

#[path = "services/mod.rs"]
pub mod services;

use axum::{
    extract::MatchedPath,
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{
    future::ready,
    net::SocketAddr,
    time::{Duration, Instant},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use consumers::start_consumers;
use servers::start_metrics_server;

#[tokio::main]
async fn main() {
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let formatting_layer = BunyanFormattingLayer::new(app_name, std::io::stdout);
    let subscriber = Registry::default()
        .with(EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| EnvFilter::new("DEBUG")))
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("starting app");
    tokio::spawn(start_consumers());
    tokio::join!(start_metrics_server());
}
