use chrono::{NaiveDateTime, NaiveTime, Utc};
use rusqlite::Connection;

use crate::types::Task;

pub const DB_FILENAME: &str = "tdo.db";

fn get_db_connection() -> Option<Connection> {
    let connection = Connection::open(DB_FILENAME).ok()?;

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            description    TEXT NOT NULL,
            completed      BOOL NOT NULL,
            date_added     DATETIME DEFAULT CURRENT_TIMESTAMP,
            date_modified  DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
            (),
        )
        .ok()?;

    Some(connection)
}

pub fn add_todo(task: Task) -> Option<Task> {
    let db = get_db_connection()?;

    db.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
        (&task.description, &task.completed),
    )
    .ok()?;

    let mut statement = db.prepare("SELECT * FROM tasks WHERE id = ?1").ok()?;
    statement
        .query_row([db.last_insert_rowid()], |row| Ok(Task::from_db(row)))
        .ok()?
}

pub fn remove_todo(task: Task) -> Option<Task> {
    let db = get_db_connection()?;
    db.execute("DELETE FROM tasks WHERE id = ?1", [&task.id.unwrap_or(0)])
        .ok()?;
    Some(task)
}

pub fn edit_todo(task: Task) -> Option<Task> {
    let db = get_db_connection()?;
    db.execute(
        "UPDATE tasks SET description = ?1, completed = ?2, date_modified = CURRENT_TIMESTAMP WHERE id = ?3",
        (
            &task.description,
            task.completed as i32,
            &task.id.unwrap_or(0),
        ),
    )
    .ok()?;
    Some(task)
}

pub fn get_todays_todos() -> Option<Vec<Task>> {
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let today = NaiveDateTime::new(Utc::now().date_naive(), time).to_string();

    let db = get_db_connection()?;
    let mut statement = db
        .prepare("SELECT * FROM tasks WHERE date_added >= ?1")
        .ok()?;
    let query = statement
        .query_map([today], |row| Ok(Task::from_db(row)))
        .ok()?;
    let tasks = query
        .filter_map(|task| Some(task.unwrap()?))
        .collect::<Vec<_>>();

    match tasks.len() {
        0 => None,
        _ => Some(tasks),
    }
}

pub fn get_todo(id: usize) -> Option<Task> {
    let db = get_db_connection()?;
    let mut statement = db.prepare("SELECT * FROM tasks WHERE id >= ?1").ok()?;
    statement
        .query_row([id], |row| Ok(Task::from_db(row)))
        .ok()?
}
