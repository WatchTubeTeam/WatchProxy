use actix_web::{
    web,
    HttpServer,
    HttpResponse,
    Responder,
    App,
    get,
};
use hyper::{
    client::HttpConnector,
    Body,
    Uri,
};

type Client = hyper::client::Client<HttpConnector, Body>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Client::new().clone())
            .service(root)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/test")]
async fn test(client: web::Data<Client>) -> Result<hyper::Response<Body>, hyper::Error> {
    let resp = client.get(Uri::from_static("https://manifest.watchtube.app")).await;

    resp
    /*
    match resp {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
    */
}
