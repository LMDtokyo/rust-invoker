use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::services::hero_service::Hero;

/// Хранилище выбранного героя + время последнего выбора
#[derive(Clone, Default)]
pub struct HeroState {
    /// Текущий выбранный герой
    pub selected: Arc<Mutex<Option<Hero>>>,

    /// Когда в последний раз выбирали героя (для cooldown)
    pub last_selected: Arc<Mutex<Option<Instant>>>,
}
