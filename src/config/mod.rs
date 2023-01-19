use std::{fs::create_dir_all, path::MAIN_SEPARATOR};

use dirs::config_dir;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CMD_ADD: &str = "add";
pub const CMD_REMOVE: &str = "remove";
pub const CMD_EDIT: &str = "edit";
pub const CMD_LIST: &str = "list";
pub const CMD_MARK: &str = "mark";
pub const CMD_DATABASE: &str = "database";

lazy_static! {
    pub static ref DB_PATH: String = {
        let separator = MAIN_SEPARATOR;

        config_dir()
            .map(|dir| {
                let path = format!(
                    "{}{separator}Tdo",
                    dir.into_os_string().to_str().unwrap_or(""),
                );

                match create_dir_all(&path) {
                    Ok(_) => format!("{path}{separator}tdo.db"),
                    Err(_) => "tdo.db".to_string(),
                }
            })
            .unwrap()
    };
}
