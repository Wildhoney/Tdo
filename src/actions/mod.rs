use crate::{types::Task, utils::write_todo};

pub fn add(description: &str) -> Option<Task> {
    let task = Task {
        id: None,
        description: description.to_string(),
        completed: false,
    };

    write_todo(task)
}
