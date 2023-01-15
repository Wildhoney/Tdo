use rusqlite::Connection;

use crate::types::Task;

fn get_db_connection() -> Option<Connection> {
    let connection = Connection::open("tdo.db").ok()?;

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            description   TEXT NOT NULL,
            completed     BOOL NOT NULL,
            date_modified DATETIME DEFAULT CURRENT_TIMESTAMP
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
        "UPDATE tasks SET description = ?1, completed = ?2 WHERE id = ?3",
        (
            &task.description,
            task.completed as i32,
            &task.id.unwrap_or(0),
        ),
    )
    .ok()?;
    Some(task)
}

pub fn get_todos() -> Option<Vec<Task>> {
    let db = get_db_connection()?;
    let mut statement = db.prepare("SELECT * FROM tasks").ok()?;
    let query = statement.query_map([], |row| Ok(Task::from_db(row))).ok()?;
    let tasks = query
        .filter_map(|task| Some(task.unwrap()?))
        .collect::<Vec<_>>();

    match tasks.len() {
        0 => None,
        _ => Some(tasks),
    }
}

pub fn get_todo_by_id(id: usize) -> Option<Task> {
    let tasks = get_todos().unwrap_or(vec![]);
    println!("{:?}", tasks);
    tasks.into_iter().find(|task| task.id == Some(id))
}
