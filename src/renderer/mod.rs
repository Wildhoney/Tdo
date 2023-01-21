use std::{thread, time::Duration};

use crate::{
    config::PKG_NAME,
    renderer::utils::{
        get_elapsed_time, get_length_of_longest_task_id, get_percentage_emoji, get_pluralised,
        is_overdue,
    },
    types::{GetTodos, Output, Symbols, Task},
};
use colored::*;
use figlet_rs::FIGfont;
use inflector::Inflector;
use mac_notification_sys::{
    get_bundle_identifier_or_default, send_notification, set_application, Notification,
};

use self::utils::get_symbols;

mod utils;

pub fn print(output: Output) {
    put_header();

    match output {
        Output::Add(Some(task)) => put_add_task(task),
        Output::Remove(Some(task)) => put_task_remove(task),
        Output::Edit(Some(task)) => put_task_edit(task),
        Output::List(Some(tasks)) => put_tasks_list(tasks),
        Output::Database(db_path) => put_database(db_path),
        Output::Watch(get_tasks) => put_watch(get_tasks),
        _ => println!("There appears to be a problem."),
    }

    put_footer();
}

fn put_header() {
    let font = FIGfont::standard().unwrap();
    let logo = font.convert("Tdo.").unwrap();
    println!("{}", logo);
}

fn put_footer() {
    print!("\n");
}

fn put_add_task(task: Task) {
    let Symbols {
        bullet, spacing, ..
    } = get_symbols();
    println!(
        "{spacing}{}: {}",
        format!("{bullet} You have added a new task"),
        task.description.dimmed()
    );
}

fn put_task_remove(task: Task) {
    let Symbols {
        bullet, spacing, ..
    } = get_symbols();
    println!(
        "{spacing}{}: {}",
        format!("{bullet} You have removed a task"),
        task.description.dimmed()
    );
}

fn put_task_edit(task: Task) {
    let Symbols {
        bullet, spacing, ..
    } = get_symbols();
    println!(
        "{spacing}{}: {}",
        format!("{bullet} You have edited a task"),
        task.description.dimmed()
    );
}

fn put_tasks_list(tasks: Vec<Task>) {
    let Symbols {
        dot,
        bullet,
        tick,
        spacing,
        lightbulb,
    } = get_symbols();
    let longest_id = get_length_of_longest_task_id(&tasks);

    let completed_count = tasks
        .iter()
        .fold(0, |count, task| count + (task.completed as usize));
    let completed_percentage = (completed_count as f64 / tasks.len() as f64) * 100.0;

    if tasks.len() == 0 {
        println!("{spacing}{bullet} There are no tasks available to show you! 😧\n");
        println!(
            "{spacing} {lightbulb} {}{}{}{}{} ",
            "You may use ".dimmed(),
            "tdo ls upcoming".bright_white(),
            " to show futures todos and ".dimmed(),
            "tdo add <description>".bright_white(),
            " to add more.".dimmed()
        );
        return;
    }

    println!(
        " You've completed {} {} which equates to {} {}\n\n",
        completed_count.to_string().bold(),
        get_pluralised("task", completed_count as i64),
        format!("{:.0}%", completed_percentage).cyan().bold(),
        get_percentage_emoji(completed_percentage)
    );

    for task in tasks {
        let id = format!(
            "{:>width$}{dot}",
            task.id.unwrap_or(0).to_string().dimmed(),
            width = longest_id
        );
        let icon = match task.completed {
            true => tick.clone(),
            false => bullet.clone(),
        };

        print!("{spacing}{icon} {id} {}", task.description);

        if let Some(date_for) = task.date_for {
            if is_overdue(date_for) {
                print!(
                    " {}",
                    format!(" {}{} ", "overdue: ", get_elapsed_time(date_for))
                        .on_bright_red()
                        .black()
                );
            }
        }

        print!("\n");

        if let Some(date_added) = task.date_added {
            print!(
                "{}",
                format!(
                    "{spacing}{}    Added {}",
                    " ".repeat(longest_id),
                    get_elapsed_time(date_added)
                )
                .dimmed()
            );

            if let Some(date_modified) = task.date_modified {
                print!(
                    "{}",
                    format!(" (edited {})", get_elapsed_time(date_modified)).dimmed()
                );
            }

            print!("\n");
        }

        print!("\n");
    }
}

fn put_database(path: String) {
    let Symbols {
        spacing, lightbulb, ..
    } = get_symbols();

    println!("{spacing} Location: {}\n", path.bold());

    println!(
        "{spacing} {lightbulb} {}",
        "You could open the SQLite DB file in an application such as TablePlus!".dimmed()
    )
}

fn put_watch(get_todos: GetTodos) {
    let Symbols {
        spacing, lightbulb, ..
    } = get_symbols();

    println!(
        "{spacing} {lightbulb} {}",
        "We will send notifications to remind you when todos need to be done.".dimmed()
    );

    let app_name = PKG_NAME.to_title_case();
    let delay = Duration::from_secs(10_800);
    let bundle = get_bundle_identifier_or_default(PKG_NAME);
    set_application(&bundle).unwrap();

    loop {
        thread::sleep(delay);

        let todo_count = get_todos()
            .unwrap_or(vec![])
            .iter()
            .filter(|task| task.completed == false)
            .collect::<Vec<_>>()
            .len();

        if todo_count > 0 {
            send_notification(
                &app_name.to_string(),
                None,
                &format!(
                    "You have {} {} to complete!",
                    get_pluralised("task", todo_count as i64),
                    todo_count
                ),
                Some(Notification::new().sound("Submarine")),
            )
            .unwrap();
        }
    }
}
