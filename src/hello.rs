use actix_web::get;
use actix_web::{HttpRequest, Responder};

#[get("/hello")]
async fn get(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}
