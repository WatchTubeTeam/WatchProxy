use axum::{
    extract::Extension,
    response::{IntoResponse, Response},
    routing, Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use std::net::SocketAddr;

use anyhow::Context;

use watch_proxy::types::ProxyError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let http_client = reqwest::Client::new();

    let app = Router::new().route("/", routing::get(root)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(http_client)),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("test debug");
    tracing::warn!("test warn");
    tracing::trace!("test trace");
    tracing::info!("Listening on {addr}");

    axum::Server::try_bind(&addr)
        .context(format!("Failed to bind to {}", addr))?
        .serve(app.into_make_service())
        .await
        .context("Failed to start web server for some reason")
}

pub async fn root(
    Extension(client): Extension<reqwest::Client>,
) -> Result<impl IntoResponse, ProxyError> {
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(
            client
                .get("https://manifest.watchtube.app")
                .send()
                .await?
                .text()
                .await?,
        )
        .unwrap())
}
