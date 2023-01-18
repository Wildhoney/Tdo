use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use clap::{arg, Arg, Command};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "remove";
pub const CMD_EDIT: &str = "edit";
pub const CMD_LIST_TODAY: &str = "list";
pub const CMD_LIST_UPCOMING: &str = "upcoming";
pub const CMD_COMPLETE: &str = "complete";
pub const CMD_INCOMPLETE: &str = "incomplete";

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
                .arg(arg!(<DESCRIPTION> "Description of the task that needs to be done today"))
                .arg_required_else_help(true)
                .arg(Arg::new("for").short('f').long("for").required(false)),
        )
        .subcommand(
            Command::new(CMD_REMOVE)
                .alias("rm")
                .about("Remove a task from your list")
                .arg(arg!(<ID> "ID of the task that no longer needs to be done today"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_EDIT)
                .about("Edit a task from within your list")
                .arg(arg!(<ID> "ID of the task that you're editing"))
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
            Command::new(CMD_COMPLETE)
                .about("Mark a task from within your list as complete")
                .arg(arg!(<ID> "ID of the task that you've completed"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_INCOMPLETE)
                .about("Mark a task from within your list as incomplete")
                .arg(arg!(<ID> "ID of the task that you've not completed"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_LIST_TODAY)
                .alias("ls")
                .about("List out all of the tasks to be done today"),
        )
        .subcommand(
            Command::new(CMD_LIST_UPCOMING)
                .alias("u")
                .about("List out all of the tasks to be tackled after today"),
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
