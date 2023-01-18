use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use clap::{arg, Arg, Command};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "remove";
pub const CMD_EDIT: &str = "edit";
pub const CMD_LIST: &str = "list";
pub const CMD_MARK: &str = "mark";
pub const CMD_DATABASE: &str = "database";

pub fn get_args() -> Command {
    Command::new(PKG_NAME)
        .version(PKG_VERSION)
        .about("Terminal based todo app for managing today's tasks with gentle reminders.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new(CMD_ADD)
                .about("Add a task to your list")
                .arg(arg!(<description> "Description of the task."))
                .arg_required_else_help(true)
                .arg(Arg::new("for").short('f').long("for").required(false)),
        )
        .subcommand(
            Command::new(CMD_REMOVE)
                .about("Remove a task from your list")
                .alias("rm")
                .alias("del")
                .alias("delete")
                .arg(arg!(<id> "ID of the task to remove."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_EDIT)
                .about("Edit a task from within your list")
                .alias("modify")
                .alias("update")
                .alias("change")
                .arg(arg!(<id> "ID of the task to edit."))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .required(false),
                )
                .arg(
                    Arg::new("completed")
                        .short('c')
                        .long("completed")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new(CMD_MARK)
                .about("Mark a task as complete or incomplete.")
                .alias("set")
                .arg(arg!(<id> "ID of the task that you've completed"))
                .arg_required_else_help(true)
                .subcommand(Command::new("complete").alias("done"))
                .subcommand(Command::new("incomplete").alias("not-done")),
        )
        .subcommand(
            Command::new(CMD_LIST)
                .alias("ls")
                .about("List all of the tasks.")
                .subcommand(Command::new("today"))
                .subcommand(Command::new("upcoming").alias("future"))
                .subcommand_required(false),
        )
        .subcommand(
            Command::new(CMD_DATABASE)
                .alias("db")
                .about("Find the location of the tdo database."),
        )
}

pub fn parse_date_from_string(date: Option<&String>) -> Option<NaiveDateTime> {
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();

    if let Some(date) = date {
        return match date.as_str() {
            "today" => Some(NaiveDateTime::new(Utc::now().date_naive(), time)),
            "tomorrow" => Some(NaiveDateTime::new(
                Utc::now().date_naive() + Duration::days(1),
                time,
            )),
            "overmorrow" => Some(NaiveDateTime::new(
                Utc::now().date_naive() + Duration::days(2),
                time,
            )),
            _ => None,
        };
    }

    None
}
