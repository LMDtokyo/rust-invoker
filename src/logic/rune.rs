use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Rune {
    pub name: String,
    pub description: String,
}

pub fn get_random_rune() -> Rune {
    let runes = vec![
        Rune {
            name: "haste".to_string(),
            description: "Doubles your speed for a short time.".to_string(),
        },
        Rune {
            name: "illusion".to_string(),
            description: "Creates illusions to confuse your enemies.".to_string(),
        },
        Rune {
            name: "bounty".to_string(),
            description: "Rewards you with gold and wisdom.".to_string(),
        },
        Rune {
            name: "arcane".to_string(),
            description: "Reduces ability cooldowns drastically.".to_string(),
        },
        Rune {
            name: "regeneration".to_string(),
            description: "Restores health and mana rapidly.".to_string(),
        },
    ];

    runes
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap_or_else(|| Rune {
            name: "unknown".to_string(),
            description: "This rune cannot be identified.".to_string(),
        })
}
