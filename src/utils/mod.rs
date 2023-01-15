use crate::types::Symbols;

use chrono::{NaiveDateTime, Utc};
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

pub fn get_elapsed_time(date: NaiveDateTime) -> String {
    let now = Utc::now().naive_utc();
    let difference = now - date;

    let (suffix, value) = match (
        difference.num_days(),
        difference.num_hours(),
        difference.num_minutes(),
        difference.num_seconds(),
    ) {
        (_, _, 0, seconds) => (get_pluralised("second", seconds), seconds),
        (_, 0, minutes, _) => (get_pluralised("minute", minutes), minutes),
        (days, 24, _, _) => (get_pluralised("day", days), days),
        (_, hours, _, _) => (get_pluralised("hour", hours), hours),
    };

    return format!("{value} {suffix}");
}

fn get_pluralised(word: &str, count: i64) -> String {
    match count {
        1 => word.to_string(),
        _ => format!("{}s", word),
    }
}
