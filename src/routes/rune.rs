use axum::response::IntoResponse;
use axum::Json;
use crate::logic::rune::{get_random_rune, Rune};

pub async fn fetch_rune() -> impl IntoResponse {
    let rune: Rune = get_random_rune();
    Json(rune)
}
