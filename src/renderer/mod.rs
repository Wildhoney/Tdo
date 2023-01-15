use crate::{
    types::{Output, Symbols, Task},
    utils::{get_percentage_emoji, get_symbols},
};
use chrono::Utc;
use colored::*;
use figlet_rs::FIGfont;

pub fn print(output: Output) -> () {
    put_header();

    match output {
        Output::Add(Some(task)) => put_add_task(task),
        Output::Remove(Some(task)) => put_task_remove(task),
        Output::Edit(Some(task)) => put_task_edit(task),
        Output::List(Some(tasks)) => put_tasks_list(tasks),
        _ => println!("There appears to be a problem."),
    }

    put_footer();
}

fn put_header() -> () {
    let font = FIGfont::standard().unwrap();
    let logo = font.convert("Tdo.").unwrap();
    println!("{}", logo);
}

fn put_footer() -> () {
    print!("\n");
}

fn put_add_task(task: Task) -> () {
    let Symbols { bullet, .. } = get_symbols();
    println!("{bullet} Added: {}", task.description)
}

fn put_task_remove(task: Task) -> () {
    let Symbols { bullet, .. } = get_symbols();
    println!("{bullet} Removed: {}", task.description)
}

fn put_task_edit(task: Task) -> () {
    let Symbols { bullet, .. } = get_symbols();
    println!("{bullet} Edited: {}", task.description)
}

fn put_tasks_list(tasks: Vec<Task>) -> () {
    let completed_count = tasks
        .iter()
        .fold(0, |count, task| count + (task.completed as usize));
    let completed_percentage = (completed_count as f64 / tasks.len() as f64) * 100.0;

    println!(
        "You've completed {} tasks which equates to {} {}\n",
        completed_count.to_string().bold(),
        format!("{:.0}%", completed_percentage).cyan().bold(),
        get_percentage_emoji(completed_percentage)
    );

    for task in tasks {
        let Symbols { dot, bullet, tick } = get_symbols();
        let id = format!("{}{dot}", task.id.unwrap_or(0).to_string().dimmed());
        let icon = match task.completed {
            true => tick,
            false => bullet,
        };

        println!("{icon} {id} {}", task.description);

        if let Some(date_modified) = task.date_modified {
            let now = Utc::now().time();
            let diff = date_modified.time() - now;
            println!(
                "{}",
                format!("     Updated {} hours ago", diff.num_hours()).dimmed()
            );
        }

        print!("\n");
    }
}
