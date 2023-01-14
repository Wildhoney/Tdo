use crate::types::{Output, Task};
use colored::*;
use figlet_rs::FIGfont;

pub fn print(output: Output) -> () {
    let font = FIGfont::standard().unwrap();
    let logo = font.convert("Tdo.").unwrap();
    println!("{}", logo);

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
        let bullet = "◦".dimmed();
        let id = format!(
            "{}{}",
            task.id.unwrap_or(0).to_string().dimmed(),
            ".".dimmed()
        );

        println!("{bullet} {id} {}", task.description);
    }
}
