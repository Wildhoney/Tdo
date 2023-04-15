use dirs::config_dir;
use inflector::Inflector;
use std::{fs::create_dir_all, path::MAIN_SEPARATOR};

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "remove";
pub const CMD_EDIT: &str = "edit";
pub const CMD_LIST: &str = "list";
pub const CMD_MARK: &str = "mark";
pub const CMD_DATABASE: &str = "database";
pub const CMD_WATCH: &str = "watch";
pub const CMD_RANDOM_TASK: &str = "task";

lazy_static! {
    pub static ref DB_PATH: String = {
        let directory_name = PKG_NAME.to_title_case();

        config_dir()
            .map(|dir| {
                let path = format!(
                    "{}{MAIN_SEPARATOR}{directory_name}",
                    dir.into_os_string().to_str().unwrap_or(""),
                );

                match create_dir_all(&path) {
                    Ok(_) => format!("{path}{MAIN_SEPARATOR}{PKG_NAME}.db"),
                    Err(_) => format!("{PKG_NAME}.db"),
                }
            })
            .unwrap()
    };
}
