use axum::{extract::Extension, response::IntoResponse, routing, Router};
use http::StatusCode;
use hyper::{client::HttpConnector, Body, Client, Uri};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let app = Router::new()
        .route("/", routing::get(root))
        .layer(Extension(client));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root<'a>(
    Extension(client): Extension<Client<HttpsConnector<HttpConnector>>>,
) -> Result<hyper::Response<Body>, (StatusCode, &'static str)> {
    match client
        .get(Uri::from_static("https://manifest.watchtube.app"))
        .await
    {
        Ok(resp) => Ok(resp),
        Err(err) => {
            tracing::warn!("Error fetching WatchTube manifest: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error fetching manifest"))
        }
    }
}
