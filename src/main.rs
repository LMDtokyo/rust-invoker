// === main.rs ===
use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

mod routes;
mod logic;
mod state;
mod services;

use routes::hero::{select_hero, get_selected_hero};
use routes::greeting::get_greeting;
use routes::rune::fetch_rune;
use routes::spell::spell_routes;
use state::app_state::AppState;
use state::hero_state::HeroState;
use state::spell_state::SpellState;

#[tokio::main]
async fn main() {
    let app_state = AppState {
        hero: HeroState::default(),
        spell: SpellState::default(),
    };

    let app = Router::new()
        .route("/greeting", get(get_greeting))
        .route("/rune", get(fetch_rune))
        .route("/hero/select", post(select_hero))
        .route("/hero/active", get(get_selected_hero))
        .merge(spell_routes())
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🦀 Server running at http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
