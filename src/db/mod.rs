use rusqlite::Connection;

use crate::types::Task;

fn get_db_connection() -> Option<Connection> {
    let connection = Connection::open("tdo.db").ok()?;

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
    let query = statement.query_row([db.last_insert_rowid()], |row| Ok(Task::from_db(row)));
    query.unwrap()
}

pub fn remove_todo(task: Task) -> Option<Task> {
    let db = get_db_connection()?;
    db.execute("DELETE FROM tasks WHERE id = ?1", [&task.id.unwrap_or(0)])
        .ok()?;
    Some(task)
}

pub fn get_todos() -> Option<Vec<Task>> {
    let db = get_db_connection()?;
    let mut statement = db.prepare("SELECT * FROM tasks").ok()?;
    let query = statement.query_map([], |row| Ok(Task::from_db(row))).ok()?;
    Some(query.map(|task| task.unwrap().unwrap()).collect::<Vec<_>>())
}

pub fn get_todo_by_id(id: usize) -> Option<Task> {
    let tasks = get_todos().unwrap_or(vec![]);
    tasks.into_iter().find(|task| task.id == Some(id))
}
