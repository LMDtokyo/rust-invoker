// === state/app_state.rs ===
use super::{hero_state::HeroState, spell_state::SpellState};

#[derive(Clone)]
pub struct AppState {
    pub hero: HeroState,
    pub spell: SpellState,
}
