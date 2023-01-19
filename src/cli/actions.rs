use clap::ArgMatches;

use crate::{
    config::DB_PATH,
    db::{add_todo, edit_todo, get_todo, get_todos, remove_todo},
    types::{DbFile, GetTodos, Task},
};

use super::utils::parse_date_from_string;

pub fn add(arg: &ArgMatches) -> Option<Task> {
    let description = arg.get_one::<String>("description").unwrap();
    let date_for = arg.get_one::<String>("for");

    let task = Task::new(description.to_string(), parse_date_from_string(date_for));
    DbFile::new().and_then(|db| add_todo(&db, task))
}

pub fn remove(arg: &ArgMatches) -> Option<Task> {
    let id = arg
        .get_one::<String>("id")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    DbFile::new().and_then(|db| {
        let task = get_todo(&db, id)?;
        remove_todo(&db, task)
    })
}

pub fn edit(arg: &ArgMatches) -> Option<Task> {
    let id = arg
        .get_one::<String>("id")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let description = arg.get_one::<String>("description");

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        task.description = description.unwrap_or(&task.description).to_owned();
        edit_todo(&db, task)
    })
}

pub fn mark(arg: &ArgMatches) -> Option<Task> {
    let id = arg
        .get_one::<String>("id")
        .unwrap()
        .parse::<usize>()
        .unwrap();

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
        Some(("today", _)) | None => get_todos(GetTodos::Today, &db),
        Some(("upcoming", _)) => get_todos(GetTodos::Upcoming, &db),
        _ => return None,
    })
}

pub fn database() -> String {
    DB_PATH.to_string()
}
