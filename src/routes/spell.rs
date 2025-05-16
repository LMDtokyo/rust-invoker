use axum::{extract::State, Json};
use axum::routing::{get, post};
use axum::Router;
use std::time::Instant;

use crate::logic::spell::{get_random_spell, Spell, CastResult, get_rank};
use crate::state::app_state::AppState;

pub fn spell_routes() -> Router<AppState> {
    Router::new()
        .route("/spell", get(get_spell))
        .route("/cast", post(cast_spell)) // можно удалить позже
        .route("/input", post(handle_input))
        .route("/invoke", post(handle_invoke))
        .route("/input/buffer", get(get_input_buffer)) // 👈 добавлено
}

async fn get_spell(State(state): State<AppState>) -> Json<Spell> {
    let mut spell_lock = state.spell.current.lock().unwrap();
    state
        .spell
        .start_time
        .lock()
        .unwrap()
        .replace(Instant::now());

    // очищаем буфер ввода при старте
    state.spell.input_buffer.lock().unwrap().clear();

    if spell_lock.is_none() {
        let new = get_random_spell();
        *spell_lock = Some(new.clone());
        return Json(new);
    }

    Json(spell_lock.clone().unwrap())
}

async fn handle_input(
    State(state): State<AppState>,
    Json(key): Json<String>,
) -> Json<()> {
    let mut buffer = state.spell.input_buffer.lock().unwrap();

    if buffer.len() >= 3 {
        buffer.clear();
    }

    if ["Q", "W", "E"].contains(&key.as_str()) {
        buffer.push(key);
    }

    Json(())
}

async fn handle_invoke(State(state): State<AppState>) -> Json<Option<CastResult>> {
    let mut buffer = state.spell.input_buffer.lock().unwrap();
    let combo = buffer.join("");

    let mut spell_lock = state.spell.current.lock().unwrap();
    let start_time = state.spell.start_time.lock().unwrap().take();

    if let Some(current) = &*spell_lock {
        if current.combo == combo {
            let elapsed = start_time
                .map(|t| t.elapsed().as_millis())
                .unwrap_or(9999);

            let rank = get_rank(elapsed);
            let new_spell = get_random_spell();
            *spell_lock = Some(new_spell.clone());
            buffer.clear();

            return Json(Some(CastResult {
                success: true,
                rank: rank.to_string(),
                cast_time_ms: elapsed,
                spell: new_spell,
            }));
        } else {
            buffer.clear();
            return Json(Some(CastResult {
                success: false,
                rank: "None".to_string(),
                cast_time_ms: 0,
                spell: current.clone(),
            }));
        }
    }

    Json(None)
}

async fn cast_spell(
    State(state): State<AppState>,
    Json(input): Json<String>,
) -> Json<Option<CastResult>> {
    let mut spell_lock = state.spell.current.lock().unwrap();
    let start_time = state.spell.start_time.lock().unwrap().take();

    if let Some(current) = &*spell_lock {
        if current.combo == input {
            let elapsed = start_time
                .map(|t| t.elapsed().as_millis())
                .unwrap_or(9999);

            let rank = get_rank(elapsed);
            let new_spell = get_random_spell();
            *spell_lock = Some(new_spell.clone());

            return Json(Some(CastResult {
                success: true,
                rank: rank.to_string(),
                cast_time_ms: elapsed,
                spell: new_spell,
            }));
        } else {
            return Json(Some(CastResult {
                success: false,
                rank: "None".to_string(),
                cast_time_ms: 0,
                spell: current.clone(),
            }));
        }
    }

    Json(None)
}

// 🧠 Новый эндпоинт для получения буфера орбов
async fn get_input_buffer(State(state): State<AppState>) -> Json<Vec<String>> {
    let buffer = state.spell.input_buffer.lock().unwrap().clone();
    Json(buffer)
}
