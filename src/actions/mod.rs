use crate::{
    db::{add_todo, get_todo_by_id, get_todos, remove_todo},
    types::Task,
};

pub fn add(description: &str) -> Option<Task> {
    let task = Task::new(description.to_string());
    add_todo(task)
}

pub fn remove(id: usize) -> Option<Task> {
    let task = get_todo_by_id(id)?;
    remove_todo(task)
}

pub fn list() -> Option<Vec<Task>> {
    println!("{:?}", get_todos());
    get_todos()
}
