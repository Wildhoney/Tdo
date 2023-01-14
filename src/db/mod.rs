use rusqlite::Connection;

use crate::types::Task;

pub fn add_todo(task: Task) -> Option<Task> {
    let connection = Connection::open("tdo.db").unwrap();

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            completed   BOOL NOT NULL
        )",
            (),
        )
        .ok()?;

    connection
        .execute(
            "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
            (&task.description, &task.completed),
        )
        .ok()?;

    let mut statement = connection
        .prepare("SELECT * FROM tasks WHERE id = ?1")
        .ok()?;

    let mut tasks = statement
        .query_map([connection.last_insert_rowid()], |row| {
            Ok(Task {
                id: Some(row.get(0)?),
                description: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .ok()?;

    tasks.next().unwrap().ok()
}

pub fn remove_todo(task: Task) -> Option<Task> {
    let connection = Connection::open("tdo.db").unwrap();

    connection
        .execute("DELETE FROM tasks WHERE id = ?1", [&task.id.unwrap_or(0)])
        .ok()?;

    Some(task)
}
