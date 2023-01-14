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
    pub date_modified: Option<String>,
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
        Some(Self {
            id: row.get(0).ok()?,
            description: row.get(1).ok()?,
            completed: row.get(2).ok()?,
            date_modified: row.get(3).ok()?,
        })
    }
}
