use axum::{response::{IntoResponse,Response}, Json};
use http::StatusCode;

use serde_json::json;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("Outbound request failed")]
    OutboundRequestFailure(#[from] reqwest::Error),
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        let (status, error_body) = match self {
            ProxyError::OutboundRequestFailure(_err) => (
                StatusCode::BAD_GATEWAY,
                "Error while making a request to YouTube backend.",
            ),
        };

        let body = Json(json!({ "error": error_body }));
        (status, body).into_response()
    }
}
