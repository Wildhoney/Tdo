use clap::ArgMatches;

use crate::{
    config::DB_PATH,
    db::{add_todo, edit_todo, get_random_todo, get_todo, get_todos, remove_todo},
    types::{DbFile, Task, TodosFor},
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

pub fn mark(arg: &ArgMatches) -> Option<Task> {
    let id = get_id_from_args(arg)?;

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        let completed = match arg.subcommand() {
            Some(("complete", _)) => true,
            Some(("incomplete", _)) => false,
            _ => return None,
        };

        task.completed = completed;
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
