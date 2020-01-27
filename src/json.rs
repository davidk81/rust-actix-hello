use actix_web::get;
use actix_web::{web, HttpRequest, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[get("/json")]
async fn get(_req: HttpRequest) -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}
