use clap::{arg, Command};

use crate::types::Output;

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "remove";
pub const CMD_LIST: &str = "list";

pub fn run() -> Output {
    match get_args().get_matches().subcommand() {
        Some((CMD_ADD, _)) => Output::Add,
        Some((CMD_REMOVE, _)) => Output::Remove,
        Some((CMD_LIST, _)) => Output::List,
        None | Some((_, _)) => Output::Unactionable,
    }
}

pub fn get_args() -> Command {
    Command::new("tdo")
        .about("Terminal based todo app for managing today's tasks with gentle reminders.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new(CMD_ADD)
                .about("Add a task to your list")
                .arg(arg!(<DESCRIPTION> "Description of the task that needs to be done today"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_REMOVE)
                .alias("rm")
                .about("Remove a task from your list")
                .arg(arg!(<ID> "ID of the task that no longer needs to be done today"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(CMD_LIST)
                .alias("ls")
                .about("List out all of the tasks to be done today"),
        )
}
