use chrono::NaiveDateTime;
use rusqlite::{Connection, Row};

use crate::config::DB_PATH;

use self::utils::parse_date_from_row;

mod utils;

pub type GetTodos = Box<dyn Fn() -> Option<Vec<Task>>>;

pub enum Output {
    Add(Option<Task>),
    Remove(Option<Task>),
    Edit(Option<Task>),
    List(Option<Vec<Task>>),
    RandomTask(Option<Task>),
    Database(String),
    Watch(GetTodos),
    Unactionable,
}

pub struct Symbols {
    pub dot: String,
    pub bullet: String,
    pub tick: String,
    pub in_progress: String,
    pub spacing: String,
    pub lightbulb: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Done => "done",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "done" => TaskStatus::Done,
            "in_progress" => TaskStatus::InProgress,
            _ => TaskStatus::Todo,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub id: Option<usize>,
    pub description: String,
    pub status: TaskStatus,
    pub date_for: Option<NaiveDateTime>,
    pub date_added: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(description: String, date_for: Option<NaiveDateTime>) -> Self {
        Self {
            id: None,
            description,
            status: TaskStatus::Todo,
            date_for,
            date_added: None,
            date_modified: None,
        }
    }

    pub fn from_db(row: &Row) -> Option<Self> {
        let description = row.get(1).ok().unwrap_or("".to_string());
        let date_for = parse_date_from_row(row.get(2).ok() as Option<String>);
        let date_added = parse_date_from_row(row.get(3).ok() as Option<String>);
        let date_modified = parse_date_from_row(row.get(4).ok() as Option<String>);
        let status = row
            .get::<_, String>(5)
            .ok()
            .map(|value| TaskStatus::from_str(&value))
            .unwrap_or(TaskStatus::Todo);

        Some(Self {
            id: row.get(0).ok(),
            description,
            status,
            date_for,
            date_added,
            date_modified,
        })
    }
}

#[derive(Clone)]
pub struct DbFile {}

#[derive(Clone)]
pub struct DbMemory {}

impl DbFile {
    pub fn new() -> Option<Connection> {
        Some(Connection::open(DB_PATH.as_str()).ok()?)
    }
}

impl DbMemory {
    #[allow(dead_code)]
    pub fn new() -> Option<Connection> {
        Some(Connection::open_in_memory().ok()?)
    }
}

#[derive(Debug)]
pub enum TodosFor {
    Today,
    Upcoming,
}

#[cfg(test)]
mod tests {
    use super::TaskStatus;

    #[test]
    fn it_round_trips_task_status() {
        for status in [TaskStatus::Todo, TaskStatus::InProgress, TaskStatus::Done] {
            assert_eq!(TaskStatus::from_str(status.as_str()), status);
        }
    }

    #[test]
    fn it_falls_back_to_todo_for_unknown_status() {
        assert_eq!(TaskStatus::from_str(""), TaskStatus::Todo);
        assert_eq!(TaskStatus::from_str("unrecognised"), TaskStatus::Todo);
    }
}
