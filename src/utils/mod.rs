use crate::types::Symbols;

use colored::*;

pub fn get_symbols() -> Symbols {
    Symbols {
        dot: ".".dimmed().to_string(),
        bullet: "◦".dimmed().to_string(),
    }
}

pub fn get_percentage_emoji(completed_percentage: f64) -> String {
    if completed_percentage >= 0.0 && completed_percentage <= 0.1 {
        return "😭".to_string();
    }

    if completed_percentage >= 0.1 && completed_percentage <= 0.25 {
        return "😣".to_string();
    }

    if completed_percentage >= 0.25 && completed_percentage <= 0.5 {
        return "😞".to_string();
    }

    if completed_percentage >= 0.5 && completed_percentage <= 0.75 {
        return "🥹".to_string();
    }

    return "🥳".to_string();
}
