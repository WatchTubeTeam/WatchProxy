use rocket::response::status;
use rocket::http::Status;
use rocket::State;

use rocket::serde::Serialize;
use rocket::serde::json::{json, Json};


#[rocket::launch]
fn rocket() -> _ {
    let http_client = reqwest::Client::new();

    rocket::build()
        .mount("/", rocket::routes![
            proxy,
            test,
        ])
        .manage(http_client)
}

#[rocket::get("/test/<n>")]
fn test(n: u8) -> Result<&'static str, status::BadRequest<&'static str>> {
    if n == 5 {
        Err(status::BadRequest(Some("hello")))
    } else {
        Ok("Hello world")
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RootResponse {
    status: u16,
    text: String,
}
#[rocket::get("/proxy?<url>")]
async fn proxy(
    http_client: &State<reqwest::Client>,
    url: String,
) -> Result<String, status::Custom<Json<RootResponse>>> {
    println!("http_client: {:?}", http_client);
    match http_client.get(url).send().await {
        Ok(resp) => {
            match resp.text().await {
                Ok(txt) => Ok(txt),
                Err(err) => Err(
                    status::Custom(Status::NotImplemented, Json(RootResponse {
                        status: 9999,
                        text: err.to_string(),
                    }))
                )
            }
        },
        Err(resp) => Err(
            status::Custom(Status::InternalServerError, Json(RootResponse {
                status: match resp.status() {
                    Some(code) => code.into(),
                    None => 0,
                },
                text: resp.to_string(),
            }))
        ),
    }
}
