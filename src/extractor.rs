use actix_web::{web, post, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    username: String,
}

#[post("/extractor")]
pub async fn post(
    body: web::Json<Event>
    ) -> Result<String> {
    Ok(format!("hello, {}\n", body.username))
}
