use crate::types::Output;

use self::{
    actions::{add, edit, list_today, list_upcoming, mark, remove},
    utils::{get_args, CMD_ADD, CMD_EDIT, CMD_LIST_TODAY, CMD_LIST_UPCOMING, CMD_MARK, CMD_REMOVE},
};

mod actions;
mod utils;

pub fn run() -> Output {
    match get_args().get_matches().subcommand() {
        Some((CMD_ADD, arg)) => Output::Add(add(arg)),
        Some((CMD_REMOVE, arg)) => Output::Remove(remove(arg)),
        Some((CMD_EDIT, arg)) => Output::Edit(edit(arg)),
        Some((CMD_MARK, arg)) => Output::Edit(mark(arg)),
        Some((CMD_LIST_TODAY, _)) => Output::List(list_today()),
        Some((CMD_LIST_UPCOMING, _)) => Output::List(list_upcoming()),
        None | Some((_, _)) => Output::Unactionable,
    }
}
