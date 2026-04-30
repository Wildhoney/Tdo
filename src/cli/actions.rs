use chrono::{Duration, NaiveDateTime, Utc};
use clap::ArgMatches;

use crate::{
    config::DB_PATH,
    db::{add_todo, edit_todo, extend_todo, get_random_todo, get_todo, get_todos, remove_todo},
    types::{DbFile, Task, TaskStatus, TodosFor},
};

use super::utils::{get_id_from_args, parse_date_from_string};

pub fn add(arg: &ArgMatches) -> Option<Task> {
    let description = arg.get_one::<String>("description").unwrap();
    let date_for = arg.get_one::<String>("for");

    let task = Task::new(description.to_string(), parse_date_from_string(date_for));
    DbFile::new().and_then(|db| add_todo(&db, task))
}

pub fn remove(arg: &ArgMatches) -> Option<Task> {
    let id = get_id_from_args(arg)?;

    DbFile::new().and_then(|db| {
        let task = get_todo(&db, id)?;
        remove_todo(&db, task)
    })
}

pub fn edit(arg: &ArgMatches) -> Option<Task> {
    let id = get_id_from_args(arg)?;
    let description = arg.get_one::<String>("description");

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        task.description = description.unwrap_or(&task.description).to_owned();
        edit_todo(&db, task)
    })
}

pub fn extend(arg: &ArgMatches) -> Option<Task> {
    let id = get_id_from_args(arg)?;

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        task.date_for = Some(extended_date_for(task.date_for?, Utc::now().naive_local()));
        extend_todo(&db, task)
    })
}

fn extended_date_for(current: NaiveDateTime, now: NaiveDateTime) -> NaiveDateTime {
    current.max(now) + Duration::hours(24)
}

pub fn mark(arg: &ArgMatches) -> Option<Task> {
    let id = get_id_from_args(arg)?;

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        let status = match arg.subcommand() {
            Some(("complete", _)) => TaskStatus::Done,
            Some(("incomplete", _)) => TaskStatus::Todo,
            Some(("in-progress", _)) => TaskStatus::InProgress,
            _ => return None,
        };

        task.status = status;
        edit_todo(&db, task)
    })
}

pub fn list(arg: &ArgMatches) -> Option<Vec<Task>> {
    DbFile::new().and_then(|db| match arg.subcommand() {
        Some(("today", _)) | None => get_todos(TodosFor::Today, &db),
        Some(("upcoming", _)) => get_todos(TodosFor::Upcoming, &db),
        _ => return None,
    })
}

pub fn watch() -> Option<Vec<Task>> {
    DbFile::new().and_then(|db| get_todos(TodosFor::Today, &db))
}

pub fn database() -> String {
    DB_PATH.to_string()
}

pub fn random_task() -> Option<Task> {
    DbFile::new().and_then(|db| get_random_todo(&db))
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate};

    use super::extended_date_for;

    #[test]
    fn it_extends_overdue_task_from_now() {
        let two_days_ago = NaiveDate::from_ymd_opt(2026, 4, 28)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        let now = NaiveDate::from_ymd_opt(2026, 4, 30)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        assert_eq!(extended_date_for(two_days_ago, now), now + Duration::hours(24));
    }

    #[test]
    fn it_extends_future_task_from_its_existing_date() {
        let next_week = NaiveDate::from_ymd_opt(2026, 5, 7)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        let now = NaiveDate::from_ymd_opt(2026, 4, 30)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        assert_eq!(
            extended_date_for(next_week, now),
            next_week + Duration::hours(24)
        );
    }
}
