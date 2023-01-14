use crate::{
    types::{Output, Symbols, Task},
    utils::get_symbols,
};
use colored::*;
use figlet_rs::FIGfont;

pub fn print(output: Output) -> () {
    put_header();

    match output {
        Output::Add(Some(task)) => put_add_task(task),
        Output::Remove(Some(task)) => put_task_remove(task),
        Output::List(Some(tasks)) => put_tasks_list(tasks),
        _ => println!("There appears to be a problem."),
    }
}

fn put_header() -> () {
    let font = FIGfont::standard().unwrap();
    let logo = font.convert("Tdo.").unwrap();
    println!("{}", logo);
}

fn put_add_task(task: Task) -> () {
    println!("Added: {:?}", task)
}

fn put_task_remove(task: Task) -> () {
    println!("Removed: {:?}", task)
}

fn put_tasks_list(tasks: Vec<Task>) -> () {
    for task in tasks {
        let Symbols { dot, bullet } = get_symbols();
        let id = format!("{}{dot}", task.id.unwrap_or(0).to_string().dimmed());

        println!("{bullet} {id} {}", task.description);
    }
}
