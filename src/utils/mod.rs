use crate::types::Symbols;

use colored::*;

pub fn get_symbols() -> Symbols {
    Symbols {
        dot: ".".dimmed().to_string(),
        bullet: "◦".dimmed().to_string(),
        tick: "✓".bright_green().to_string(),
    }
}

pub fn get_percentage_emoji(completed_percentage: f64) -> String {
    if completed_percentage >= 0.0 && completed_percentage <= 10.0 {
        return "😭".to_string();
    }

    if completed_percentage >= 10.0 && completed_percentage <= 25.0 {
        return "😣".to_string();
    }

    if completed_percentage >= 25.0 && completed_percentage <= 50.0 {
        return "😞".to_string();
    }

    if completed_percentage >= 50.0 && completed_percentage <= 75.0 {
        return "🥹".to_string();
    }

    return "🥳".to_string();
}
