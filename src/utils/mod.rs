use crate::types::Symbols;

use colored::*;

pub fn get_symbols() -> Symbols {
    Symbols {
        dot: ".".dimmed().to_string(),
        bullet: "◦".dimmed().to_string(),
    }
}
