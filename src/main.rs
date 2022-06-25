use actix_web::{
    HttpServer,
    HttpResponse,
    Responder,
    App,
    get,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
