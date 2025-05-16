// === routes/hero.rs ===
use axum::{extract::State, Json};
use crate::state::app_state::AppState;
use crate::services::hero_service::{try_select_hero, get_selected};
use crate::services::hero_service::Hero;

pub async fn select_hero(
    State(state): State<AppState>,
    Json(hero): Json<Hero>,
) -> impl axum::response::IntoResponse {
    try_select_hero(state.hero.clone(), hero).await
}

pub async fn get_selected_hero(
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    get_selected(state.hero.clone()).await
}
