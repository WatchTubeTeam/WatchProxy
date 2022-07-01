use axum::{extract::Extension,response::{IntoResponse, Response}};

use crate::types::ProxyError;


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
