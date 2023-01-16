use crate::{
    db::{add_todo, edit_todo, get_todays_todos, get_todo, remove_todo},
    types::Task,
};

pub fn add(description: &str) -> Option<Task> {
    let task = Task::new(description.to_string());
    add_todo(task)
}

pub fn remove(id: usize) -> Option<Task> {
    let task = get_todo(id)?;
    remove_todo(task)
}

pub fn edit(id: usize, description: Option<&String>, completed: Option<bool>) -> Option<Task> {
    let mut task = get_todo(id)?;
    task.description = description.unwrap_or(&task.description).to_owned();
    task.completed = completed.unwrap_or(task.completed).to_owned();
    edit_todo(task)
}

pub fn list() -> Option<Vec<Task>> {
    get_todays_todos()
}
