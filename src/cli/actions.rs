use clap::ArgMatches;

use crate::{
    db::{add_todo, edit_todo, get_todo, get_todos, remove_todo},
    types::{DbFile, GetTodos, Task},
};

use super::utils::parse_date_from_string;

pub fn add(arg: &ArgMatches) -> Option<Task> {
    let description = arg.get_one::<String>("DESCRIPTION").unwrap();
    let date_for = arg.get_one::<String>("for");

    let task = Task::new(description.to_string(), parse_date_from_string(date_for));
    DbFile::new().and_then(|db| add_todo(&db, task))
}

pub fn remove(arg: &ArgMatches) -> Option<Task> {
    let id = arg
        .get_one::<String>("ID")
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
        .get_one::<String>("ID")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let description = arg.get_one::<String>("description");
    let completed = match arg.get_one::<String>("completed") {
        Some(value) => value.parse::<bool>().ok(),
        _ => None,
    };

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        task.description = description.unwrap_or(&task.description).to_owned();
        task.completed = completed.unwrap_or(task.completed).to_owned();
        edit_todo(&db, task)
    })
}

pub fn mark(arg: &ArgMatches, completed: bool) -> Option<Task> {
    let id = arg
        .get_one::<String>("ID")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        task.completed = completed;
        edit_todo(&db, task)
    })
}

pub fn list(arg: &ArgMatches) -> Option<Vec<Task>> {
    let all = match arg.get_one::<String>("all") {
        Some(value) => value.parse::<bool>().ok(),
        _ => None,
    };

    let complete = match arg.get_one::<String>("complete") {
        Some(value) => value.parse::<bool>().ok(),
        _ => None,
    };

    let incomplete = match arg.get_one::<String>("incomplete") {
        Some(value) => value.parse::<bool>().ok(),
        _ => None,
    };

    DbFile::new().and_then(|db| match (all, incomplete, complete) {
        (_, _, Some(true)) => get_todos(GetTodos::AllComplete, &db),
        (_, Some(true), _) => get_todos(GetTodos::AllIncomplete, &db),
        (Some(true), _, _) => get_todos(GetTodos::All, &db),
        _ => get_todos(GetTodos::Today, &db),
    })
}
