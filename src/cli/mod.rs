use crate::{
    config::{
        CMD_ADD, CMD_DATABASE, CMD_EDIT, CMD_LIST, CMD_MARK, CMD_RANDOM_TASK, CMD_REMOVE, CMD_WATCH,
    },
    types::Output,
};

use self::{
    actions::{add, database, edit, list, mark, random_task, remove, watch},
    utils::get_args,
};

mod actions;
mod utils;

pub fn run() -> Output {
    match get_args().get_matches().subcommand() {
        Some((CMD_ADD, arg)) => Output::Add(add(arg)),
        Some((CMD_REMOVE, arg)) => Output::Remove(remove(arg)),
        Some((CMD_EDIT, arg)) => Output::Edit(edit(arg)),
        Some((CMD_MARK, arg)) => Output::Edit(mark(arg)),
        Some((CMD_LIST, arg)) => Output::List(list(arg)),
        Some((CMD_RANDOM_TASK, _)) => Output::RandomTask(random_task()),
        Some((CMD_WATCH, _)) => Output::Watch(Box::new(watch)),
        Some((CMD_DATABASE, _)) => Output::Database(database()),
        None | Some((_, _)) => Output::Unactionable,
    }
}
