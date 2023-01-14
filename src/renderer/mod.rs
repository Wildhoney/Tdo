use crate::types::{Output, Task};

pub fn print(output: Output) -> () {
    match output {
        Output::Add(Some(task)) => println!("Added: {:?}", task),
        Output::Remove(Some(task)) => println!("Removed: {:?}", task),
        Output::List(Some(tasks)) => list(tasks),
        _ => println!("There appears to be a problem."),
    }
}

fn list(tasks: Vec<Task>) -> () {
    for task in tasks {
        println!("{}: {}", task.id.unwrap_or(0), task.description);
    }
}
