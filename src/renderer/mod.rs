use crate::types::{Output, Task};

pub fn print(output: Output) -> () {
    match output {
        Output::Add(Some(task)) => add(task),
        Output::Remove(Some(task)) => remove(task),
        Output::List(Some(tasks)) => list(tasks),
        _ => println!("There appears to be a problem."),
    }
}

fn add(task: Task) -> () {
    println!("Added: {:?}", task)
}

fn remove(task: Task) -> () {
    println!("Removed: {:?}", task)
}

fn list(tasks: Vec<Task>) -> () {
    for task in tasks {
        println!("{}: {}", task.id.unwrap_or(0), task.description);
    }
}
