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

pub fn get_time_elapsed(date_modified: NaiveDateTime) -> String {
    let now = Utc::now().naive_utc();
    let difference = now - date_modified;

    let (suffix, value) = match (
        difference.num_days(),
        difference.num_hours(),
        difference.num_minutes(),
    ) {
        (_, 0, minutes) => (get_pluralised("minute", minutes), minutes),
        (days, 24, _) => (get_pluralised("day", days), days),
        (_, hours, _) => (get_pluralised("hour", hours), hours),
    };

    return format!("{value} {suffix}");
}

fn get_pluralised(word: &str, count: i64) -> String {
    match count {
        1 => word.to_string(),
        _ => format!("{}s", word),
    }
}
