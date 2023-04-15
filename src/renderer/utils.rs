use crate::types::{Symbols, Task};

use chrono::{NaiveDateTime, NaiveTime, Utc};
use colored::*;

pub fn get_symbols() -> Symbols {
    Symbols {
        dot: ".".dimmed().to_string(),
        bullet: "◦".dimmed().to_string(),
        tick: "✓".bright_green().to_string(),
        spacing: "  ".to_string(),
        lightbulb: "💡".to_string(),
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
    let ago = (
        difference.num_minutes(),
        difference.num_hours(),
        difference.num_days(),
        difference.num_weeks(),
    );

    match ago {
        (minutes, _, _, _) if minutes < 1 => "just now".to_string(),
        (minutes, hours, _, _) if hours < 1 => {
            format!("{} {} ago", minutes, get_pluralised("minute", minutes))
        }
        (_, hours, _, _) if hours < 24 => {
            format!("{} {} ago", hours, get_pluralised("hour", hours))
        }
        (_, _, days, _) if days < 31 => format!("{} {} ago", days, get_pluralised("day", days)),
        (_, _, _, weeks) if weeks < 8 => format!("{} {} ago", weeks, get_pluralised("week", weeks)),
        _ => "a long time ago".to_string(),
    }
}

pub fn is_overdue(date: NaiveDateTime) -> bool {
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let beginning_of_today = NaiveDateTime::new(Utc::now().date_naive(), time);
    date < beginning_of_today
}

pub fn get_pluralised(word: &str, count: i64) -> String {
    match count {
        1 => word.to_string(),
        _ => format!("{}s", word),
    }
}

pub fn get_length_of_longest_task_id(tasks: &Vec<Task>) -> usize {
    tasks.iter().fold(0, |current_longest, task| {
        let id = task.id.unwrap_or(0);
        let length = id.to_string().len();

        if length > current_longest {
            length
        } else {
            current_longest
        }
    })
}

pub fn print_overdue(task: &Task) {
    if let Some(date_for) = task.date_for {
        if is_overdue(date_for) {
            if task.completed {
                print!(
                    " {}",
                    format!(" {}{} ", "overdue: ", get_elapsed_time(date_for))
                        .on_green()
                        .black()
                );
            } else {
                print!(
                    " {}",
                    format!(" {}{} ", "overdue: ", get_elapsed_time(date_for))
                        .on_bright_red()
                        .black()
                );
            }
        }
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
        let one_second_ago = Utc::now().naive_utc() - Duration::seconds(1);
        assert_eq!(get_elapsed_time(one_second_ago), "just now");
        let five_seconds_ago = Utc::now().naive_utc() - Duration::seconds(5);
        assert_eq!(get_elapsed_time(five_seconds_ago), "just now");

        let one_minute_ago = Utc::now().naive_utc() - Duration::minutes(1);
        assert_eq!(get_elapsed_time(one_minute_ago), "1 minute ago");
        let ten_minutes_ago = Utc::now().naive_utc() - Duration::minutes(10);
        assert_eq!(get_elapsed_time(ten_minutes_ago), "10 minutes ago");

        let yesterday = Utc::now().naive_utc() - Duration::days(1);
        assert_eq!(get_elapsed_time(yesterday), "1 day ago");
        let ereyesterday = Utc::now().naive_utc() - Duration::days(2);
        assert_eq!(get_elapsed_time(ereyesterday), "2 days ago");

        let five_weeks_ago = Utc::now().naive_utc() - Duration::weeks(5);
        assert_eq!(get_elapsed_time(five_weeks_ago), "5 weeks ago");

        let fourteen_weeks_ago = Utc::now().naive_utc() - Duration::weeks(14);
        assert_eq!(get_elapsed_time(fourteen_weeks_ago), "a long time ago");
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

    #[test]
    fn it_can_determine_when_overdue() {
        let one_second_ago = Utc::now().naive_utc() - Duration::seconds(1);
        assert_eq!(is_overdue(one_second_ago), false);

        let one_day_ago = Utc::now().naive_utc() - Duration::days(1);
        assert_eq!(is_overdue(one_day_ago), true);

        let six_months_ago = Utc::now().naive_utc() - Duration::weeks(26);
        assert_eq!(is_overdue(six_months_ago), true);
    }

    #[test]
    fn it_can_get_length_of_longest_task_id() {
        let mut first_task = Task::new("Adam".to_string(), Some(Utc::now().naive_local()));
        first_task.id = Some(1);

        let mut second_task = Task::new("Maria".to_string(), Some(Utc::now().naive_local()));
        second_task.id = Some(1_500);

        let mut third_task = Task::new("Imogen".to_string(), Some(Utc::now().naive_local()));
        third_task.id = Some(750);

        let tasks = vec![first_task, second_task, third_task];
        assert_eq!(get_length_of_longest_task_id(&tasks), 4)
    }
}
