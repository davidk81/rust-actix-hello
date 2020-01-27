use actix_web::post;
use actix_web::{web, Result};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct QueryParams {
    username: String,
}

#[post("/params")]
pub async fn get(
    query: web::Query<QueryParams>
    ) -> Result<String> {
    Ok(format!("Hello, {}\n", query.username))
}
