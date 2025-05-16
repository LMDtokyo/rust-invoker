use crate::state::hero_state::HeroState;
use std::time::{Duration, Instant};
use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// Структура героя, выбираемого пользователем
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub name: String,
    pub image: String,
}

/// Обрабатывает попытку выбрать героя.
/// Возвращает ошибку, если герой уже выбран менее чем минуту назад.
pub async fn try_select_hero(
    state: HeroState,
    hero: Hero,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let now = Instant::now();
    let mut last_selected = state.last_selected.lock().unwrap();

    if let Some(last_time) = *last_selected {
        if now.duration_since(last_time) < Duration::from_secs(60) {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                "You can only select a hero once per minute.",
            ));
        }
    }

    // Устанавливаем нового героя
    *state.selected.lock().unwrap() = Some(hero.clone());
    *last_selected = Some(now);

    Ok(Json(hero))
}

/// Возвращает текущего выбранного героя.
pub async fn get_selected(state: HeroState) -> impl IntoResponse {
    let selected = state.selected.lock().unwrap();
    Json(selected.clone())
}
