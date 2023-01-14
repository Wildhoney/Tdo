use crate::{
    db::{add_todo, list_todos, remove_todo},
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
    let task = Task {
        id: Some(id),
        description: "".to_string(),
        completed: false,
    };

    remove_todo(task)
}

pub fn list() -> Option<Vec<Task>> {
    list_todos()
}
