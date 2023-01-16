use chrono::NaiveDateTime;
use rusqlite::Row;

use crate::utils::parse_date_from_row;

#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Option<Task>),
    Remove(Option<Task>),
    Edit(Option<Task>),
    List(Option<Vec<Task>>),
    Unactionable,
}

pub struct Symbols {
    pub dot: String,
    pub bullet: String,
    pub tick: String,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<usize>,
    pub description: String,
    pub completed: bool,
    pub date_added: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            id: None,
            description,
            completed: false,
            date_added: None,
            date_modified: None,
        }
    }

    pub fn from_db(row: &Row) -> Option<Self> {
        let description = row.get(1).ok().unwrap_or("".to_string());
        let completed = row.get(2).ok().unwrap_or(false);
        let date_added = parse_date_from_row(row.get(3).ok() as Option<String>);
        let date_modified = parse_date_from_row(row.get(4).ok() as Option<String>);

        Some(Self {
            id: row.get(0).ok(),
            description,
            completed,
            date_added,
            date_modified,
        })
    }
}
