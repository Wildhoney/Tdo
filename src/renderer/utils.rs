use crate::types::Symbols;

use chrono::{NaiveDateTime, Utc};
use colored::*;

pub fn get_symbols() -> Symbols {
    Symbols {
        dot: ".".dimmed().to_string(),
        bullet: "◦".dimmed().to_string(),
        tick: "✓".bright_green().to_string(),
        spacing: "  ".to_string(),
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
        (days, _, _, _) if days >= 1 => (get_pluralised("day", days), days),
        (_, _, 0, seconds) => (get_pluralised("second", seconds), seconds),
        (_, 0, minutes, _) => (get_pluralised("minute", minutes), minutes),
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn it_can_pluralise_words() {
        assert_eq!(get_pluralised(&"Imogen".to_string(), 1), "Imogen");
        assert_eq!(get_pluralised(&"Imogen".to_string(), 2), "Imogens");
    }

    #[test]
    fn it_can_get_elapsed_time() {
        let yesterday = Utc::now().naive_utc() - Duration::days(1);
        assert_eq!(get_elapsed_time(yesterday), "1 day");
        let ereyesterday = Utc::now().naive_utc() - Duration::days(2);
        assert_eq!(get_elapsed_time(ereyesterday), "2 days");

        let one_second_ago = Utc::now().naive_utc() - Duration::seconds(1);
        assert_eq!(get_elapsed_time(one_second_ago), "1 second");
        let five_seconds_ago = Utc::now().naive_utc() - Duration::seconds(5);
        assert_eq!(get_elapsed_time(five_seconds_ago), "5 seconds");

        let one_minute_ago = Utc::now().naive_utc() - Duration::minutes(1);
        assert_eq!(get_elapsed_time(one_minute_ago), "1 minute");
        let ten_minutes_ago = Utc::now().naive_utc() - Duration::minutes(10);
        assert_eq!(get_elapsed_time(ten_minutes_ago), "10 minutes");
    }

    #[test]
    fn it_can_get_percentage_emoji() {
        let emoji = "😭";
        assert_eq!(get_percentage_emoji(0.0), emoji);
        assert_eq!(get_percentage_emoji(5.0), emoji);
        assert_eq!(get_percentage_emoji(10.0), emoji);

        let emoji = "😣";
        assert_eq!(get_percentage_emoji(10.1), emoji);
        assert_eq!(get_percentage_emoji(12.5), emoji);
        assert_eq!(get_percentage_emoji(25.0), emoji);

        let emoji = "😞";
        assert_eq!(get_percentage_emoji(25.1), emoji);
        assert_eq!(get_percentage_emoji(32.5), emoji);
        assert_eq!(get_percentage_emoji(50.0), emoji);

        let emoji = "🥹";
        assert_eq!(get_percentage_emoji(50.1), emoji);
        assert_eq!(get_percentage_emoji(62.5), emoji);
        assert_eq!(get_percentage_emoji(75.0), emoji);

        let emoji = "🥳";
        assert_eq!(get_percentage_emoji(75.1), emoji);
        assert_eq!(get_percentage_emoji(82.5), emoji);
        assert_eq!(get_percentage_emoji(100.0), emoji);
    }
}
