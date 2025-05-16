// === 1. logic/spell.rs ===
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub id: String,
    pub combo: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CastResult {
    pub success: bool,
    pub rank: String,
    pub cast_time_ms: u128,
    pub spell: Spell,
}

pub fn get_random_spell() -> Spell {
    let all = vec![
        Spell { id: "cold_snap".into(), combo: "QQQ".into(), image: "/spells/cold_snap.png".into() },
        Spell { id: "ghost_walk".into(), combo: "QQW".into(), image: "/spells/ghost_walk.png".into() },
        Spell { id: "deafening_blast".into(), combo: "QWE".into(), image: "/spells/deafening_blast.png".into() },
        Spell { id: "tornado".into(), combo: "QWW".into(), image: "/spells/tornado.png".into() },
        Spell { id: "emp".into(), combo: "WWW".into(), image: "/spells/emp.png".into() },
        Spell { id: "forge_spirit".into(), combo: "QEE".into(), image: "/spells/forge_spirit.png".into() },
        Spell { id: "sun_strike".into(), combo: "EEE".into(), image: "/spells/sun_strike.png".into() },
        Spell { id: "ice_wall".into(), combo: "QQE".into(), image: "/spells/ice_wall.png".into() },
        Spell { id: "alacrity".into(), combo: "WWE".into(), image: "/spells/alacrity.png".into() },
    ];
    all.choose(&mut rand::thread_rng()).unwrap().clone()
}

pub fn get_rank(ms: u128) -> &'static str {
    match ms {
        0..=900 => "Титан",
        901..=1100 => "Божество",
        1101..=1300 => "Властелин",
        1301..=1600 => "Легенда",
        1601..=2000 => "Герой",
        2001..=2500 => "Рыцарь",
        2501..=3000 => "Страж",
        _ => "Рекрут",
    }
}
