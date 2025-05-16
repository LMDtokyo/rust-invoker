use crate::logic::spell::Spell;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone, Default)]
pub struct SpellState {
    pub current: Arc<Mutex<Option<Spell>>>,
    pub start_time: Arc<Mutex<Option<Instant>>>,
    pub input_buffer: Arc<Mutex<Vec<String>>>, // 👈 добавлено
}
