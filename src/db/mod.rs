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

    let query = statement.query_row([connection.last_insert_rowid()], |row| {
        Ok(Task {
            id: Some(row.get(0)?),
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    });

    Some(query.unwrap())
}

pub fn remove_todo(task: Task) -> Option<Task> {
    let connection = Connection::open("tdo.db").unwrap();

    connection
        .execute("DELETE FROM tasks WHERE id = ?1", [&task.id.unwrap_or(0)])
        .ok()?;

    Some(task)
}

pub fn list_todos() -> Option<Vec<Task>> {
    let connection = Connection::open("tdo.db").unwrap();

    let mut statement = connection.prepare("SELECT * FROM tasks").ok()?;

    let query = statement
        .query_map([], |row| {
            Ok(Task {
                id: Some(row.get(0)?),
                description: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .ok()?;

    Some(query.map(|task| task.unwrap()).collect::<Vec<_>>())
}
