use chrono::Timelike;
use rand::seq::SliceRandom;

pub fn generate_greeting(name: &str) -> String {
    let hour = chrono::Local::now().hour();
    let part_of_day = match hour {
        5..=11 => "morning",
        12..=17 => "afternoon",
        18..=21 => "evening",
        _ => "night",
    };

    let titles = vec![
        "the Brave",
        "the Rustacean",
        "of the Ancient Forge",
        "Doomcaller",
        "Packet Wizard",
        "Memory Lord",
    ];

    let title = titles.choose(&mut rand::thread_rng()).unwrap_or(&"the Unknown");
    let power = (rand::random::<u8>() % 100) + 1;

    format!(
        "Good {}, {} {}! Your arcane energy is at {}%.",
        part_of_day, name, title, power
    )
}
