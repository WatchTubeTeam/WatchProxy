use axum::{
    routing,
    Router,
    extract::Extension,
    response,
};
use hyper::{
    Uri,
    client::HttpConnector,
    Body,
    Response,
};
use std::net::SocketAddr;

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = Client::new();

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

async fn root<'a>(Extension(client): Extension<Client>) -> response::Response {
    client.get(Uri::from_static("https://manifest.watchtube.app"))
        .await?.body()
}
