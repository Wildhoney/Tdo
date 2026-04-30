use rusqlite::{Connection, Params};

use crate::types::{Task, TaskStatus};

pub fn get_db_connection(db: &Connection) -> Option<&Connection> {
    let default_status = TaskStatus::Todo.as_str();

    db.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS tasks (
                id             INTEGER PRIMARY KEY AUTOINCREMENT,
                description    TEXT NOT NULL,
                date_for       DATETIME DEFAULT CURRENT_TIMESTAMP,
                date_added     DATETIME DEFAULT CURRENT_TIMESTAMP,
                date_modified  DATETIME,
                status         TEXT NOT NULL DEFAULT '{default_status}'
            )"
        ),
        (),
    )
    .ok()?;

    let _ = db.execute(
        &format!("ALTER TABLE tasks ADD COLUMN status TEXT NOT NULL DEFAULT '{default_status}'"),
        (),
    );
    let _ = db.execute(
        "UPDATE tasks SET status = ?1 WHERE completed = 1 AND status = ?2",
        [TaskStatus::Done.as_str(), TaskStatus::Todo.as_str()],
    );
    let _ = db.execute("ALTER TABLE tasks DROP COLUMN completed", ());

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
