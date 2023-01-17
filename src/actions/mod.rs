use crate::{
    db::{add_todo, edit_todo, get_todo, get_todos, remove_todo},
    types::{DbFile, GetTodos, Task},
};

pub fn add(description: &str) -> Option<Task> {
    let task = Task::new(description.to_string());
    DbFile::new().and_then(|db| add_todo(&db, task))
}

pub fn remove(id: usize) -> Option<Task> {
    DbFile::new().and_then(|db| {
        let task = get_todo(&db, id)?;
        remove_todo(&db, task)
    })
}

pub fn edit(id: usize, description: Option<&String>, completed: Option<bool>) -> Option<Task> {
    DbFile::new().and_then(|db| {
        let mut task = get_todo(&db, id)?;
        task.description = description.unwrap_or(&task.description).to_owned();
        task.completed = completed.unwrap_or(task.completed).to_owned();
        edit_todo(&db, task)
    })
}

pub fn list(
    all: Option<bool>,
    complete: Option<bool>,
    incomplete: Option<bool>,
) -> Option<Vec<Task>> {
    DbFile::new().and_then(|db| match (all, incomplete, complete) {
        (_, _, Some(true)) => get_todos(GetTodos::AllComplete, &db),
        (_, Some(true), _) => get_todos(GetTodos::AllIncomplete, &db),
        (Some(true), _, _) => get_todos(GetTodos::All, &db),
        _ => get_todos(GetTodos::Today, &db),
    })
}
