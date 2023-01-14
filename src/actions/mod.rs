use crate::{
    db::{add_todo, get_todo_by_id, get_todos, remove_todo},
    types::Task,
};

pub fn add(description: &str) -> Option<Task> {
    let task = Task {
        id: None,
        description: description.to_string(),
        completed: false,
    };

    add_todo(task)
}

pub fn remove(id: usize) -> Option<Task> {
    let task = get_todo_by_id(id)?;
    remove_todo(task)
}

pub fn list() -> Option<Vec<Task>> {
    get_todos()
}
