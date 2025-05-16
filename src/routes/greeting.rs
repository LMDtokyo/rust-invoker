use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::logic::greeting::generate_greeting;

#[derive(Debug, Deserialize)]
pub struct GreetParams {
    name: String,
}

pub async fn get_greeting(Query(params): Query<GreetParams>) -> impl IntoResponse {
    generate_greeting(&params.name)
}
