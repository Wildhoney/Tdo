use crate::{
    actions::{add, edit, list, remove},
    types::Output,
};

use self::utils::{
    get_args, CMD_ADD, CMD_COMPLETE, CMD_EDIT, CMD_INCOMPLETE, CMD_LIST, CMD_REMOVE,
};

mod utils;

pub fn run() -> Output {
    match get_args().get_matches().subcommand() {
        Some((CMD_ADD, arg)) => {
            let description = arg.get_one::<String>("DESCRIPTION").unwrap();
            Output::Add(add(&description))
        }
        Some((CMD_REMOVE, arg)) => {
            let id = arg
                .get_one::<String>("ID")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Output::Remove(remove(id))
        }
        Some((CMD_EDIT, arg)) => {
            let id = arg
                .get_one::<String>("ID")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let description = arg.get_one::<String>("description");
            let completed = match arg.get_one::<String>("completed") {
                Some(value) => value.parse::<bool>().ok(),
                _ => None,
            };

            Output::Edit(edit(id, description, completed))
        }
        Some((CMD_COMPLETE, arg)) => {
            let id = arg
                .get_one::<String>("ID")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Output::Edit(edit(id, None, Some(true)))
        }
        Some((CMD_INCOMPLETE, arg)) => {
            let id = arg
                .get_one::<String>("ID")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Output::Edit(edit(id, None, Some(false)))
        }
        Some((CMD_LIST, arg)) => {
            let all = match arg.get_one::<String>("all") {
                Some(value) => value.parse::<bool>().ok(),
                _ => None,
            };

            let complete = match arg.get_one::<String>("complete") {
                Some(value) => value.parse::<bool>().ok(),
                _ => None,
            };

            let incomplete = match arg.get_one::<String>("incomplete") {
                Some(value) => value.parse::<bool>().ok(),
                _ => None,
            };

            Output::List(list(all, complete, incomplete))
        }
        None | Some((_, _)) => Output::Unactionable,
    }
}
