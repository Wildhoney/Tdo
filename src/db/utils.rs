use rusqlite::{Connection, Params};

use crate::types::Task;

pub fn get_db_connection(db: &Connection) -> Option<&Connection> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            description    TEXT NOT NULL,
            completed      BOOL NOT NULL,
            date_for       DATETIME DEFAULT CURRENT_TIMESTAMP,
            date_added     DATETIME DEFAULT CURRENT_TIMESTAMP,
            date_modified  DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .ok()?;

    Some(db)
}

pub fn prepare_todos<P>(db: &Connection, query: &String, params: P) -> Option<Vec<Task>>
where
    P: Params,
{
    let db = get_db_connection(db)?;
    let mut statement = db.prepare(query).ok()?;
    let query = statement
        .query_map(params, |row| Ok(Task::from_db(row)))
        .ok()?;

    let tasks = query
        .filter_map(|task| Some(task.unwrap()?))
        .collect::<Vec<_>>();

    Some(tasks)
}
