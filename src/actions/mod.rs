use crate::{
    db::{add_todo, edit_todo, get_all_todos, get_todays_todos, get_todo, remove_todo},
    types::{DbFile, Task},
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

pub fn list(all: Option<bool>) -> Option<Vec<Task>> {
    DbFile::new().and_then(|db| match all {
        Some(true) => get_all_todos(&db),
        _ => get_todays_todos(&db),
    })
}
