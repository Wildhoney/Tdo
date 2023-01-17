use crate::{
    renderer::utils::{get_elapsed_time, get_percentage_emoji},
    types::{Output, Symbols, Task},
};
use colored::*;
use figlet_rs::FIGfont;

use self::utils::get_symbols;

mod utils;

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
    let Symbols {
        bullet, spacing, ..
    } = get_symbols();
    println!(
        "{spacing}{}: {}",
        format!("{bullet} You have added a new task"),
        task.description.dimmed()
    );
}

fn put_task_remove(task: Task) -> () {
    let Symbols {
        bullet, spacing, ..
    } = get_symbols();
    println!(
        "{spacing}{}: {}",
        format!("{bullet} You have removed a task"),
        task.description.dimmed()
    );
}

fn put_task_edit(task: Task) -> () {
    let Symbols {
        bullet, spacing, ..
    } = get_symbols();
    println!(
        "{spacing}{}: {}",
        format!("{bullet} You have edited a task"),
        task.description.dimmed()
    );
}

fn put_tasks_list(tasks: Vec<Task>) -> () {
    let Symbols {
        dot,
        bullet,
        tick,
        spacing,
        lightbulb,
    } = get_symbols();

    let completed_count = tasks
        .iter()
        .fold(0, |count, task| count + (task.completed as usize));
    let completed_percentage = (completed_count as f64 / tasks.len() as f64) * 100.0;

    if tasks.len() == 0 {
        println!("{spacing}{bullet} You haven't yet added any tasks for today!\n");
        println!(
            "{spacing} {lightbulb} {}{}{}",
            "You may use ".dimmed(),
            "tdo ls --incomplete true".bright_white(),
            " to show all of your incomplete todos.".dimmed()
        );
        return;
    }

    println!(
        "You've completed {} tasks which equates to {} {}\n",
        completed_count.to_string().bold(),
        format!("{:.0}%", completed_percentage).cyan().bold(),
        get_percentage_emoji(completed_percentage)
    );

    for task in tasks {
        let id = format!("{}{dot}", task.id.unwrap_or(0).to_string().dimmed());
        let icon = match task.completed {
            true => tick.clone(),
            false => bullet.clone(),
        };

        println!("{spacing}{icon} {id} {}", task.description);

        if let Some(date_added) = task.date_added {
            print!(
                "{}",
                format!("       Added {} ago", get_elapsed_time(date_added)).dimmed()
            );

            if let Some(date_modified) = task.date_modified {
                print!(
                    "{}",
                    format!(" (edited {} ago)", get_elapsed_time(date_modified)).dimmed()
                );
            }

            print!("\n");
        }

        print!("\n");
    }
}
