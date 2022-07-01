use axum::{extract::Extension, response::{IntoResponse, Response}, routing, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use http::StatusCode;
use std::net::SocketAddr;

use anyhow::Context;
use thiserror::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let http_client = reqwest::Client::new();

    let app = Router::new()
        .route("/", routing::get(root))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(http_client))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Error starting axum server")
}


#[derive(Error, Debug)]
enum ProxyError {
    #[error("Outbound request failed")]
    OutboundRequestFailure(#[from] reqwest::Error),
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        match self {
            ProxyError::OutboundRequestFailure(_err) => (StatusCode::BAD_GATEWAY, "hello").into_response()
        }
    }
}


async fn root(
    Extension(client): Extension<reqwest::Client>,
) -> Result<String, ProxyError> {
    Ok(client.get("https://manifest.watchtube.app").send().await?.text().await?)
}
