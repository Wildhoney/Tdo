use chrono::NaiveDateTime;
use rusqlite::Row;

#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Option<Task>),
    Remove(Option<Task>),
    List(Option<Vec<Task>>),
    Unactionable,
}

pub struct Symbols {
    pub dot: String,
    pub bullet: String,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<usize>,
    pub description: String,
    pub completed: bool,
    pub date_modified: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            id: None,
            description,
            completed: false,
            date_modified: None,
        }
    }

    pub fn from_db(row: &Row) -> Option<Self> {
        let date_modified: String = row.get(3).ok()?;
        let date_modified: NaiveDateTime =
            NaiveDateTime::parse_from_str(&date_modified, "%Y-%m-%d %H:%M:%S").ok()?;

        Some(Self {
            id: row.get(0).ok()?,
            description: row.get(1).ok()?,
            completed: row.get(2).ok()?,
            date_modified: Some(date_modified),
        })
    }
}
