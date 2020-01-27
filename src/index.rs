use actix_web::get;
use actix_web::{HttpRequest, Responder, HttpResponse};

#[get("/")]
pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
