use chrono::{NaiveDateTime, NaiveTime, Utc};
use rusqlite::{Connection, Params};

use crate::{types::Task, utils::get_db_connection};

pub fn add_todo(db: &Connection, task: Task) -> Option<Task> {
    let db = get_db_connection(db)?;

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

pub fn remove_todo(db: &Connection, task: Task) -> Option<Task> {
    let db = get_db_connection(db)?;
    db.execute("DELETE FROM tasks WHERE id = ?1", [&task.id.unwrap_or(0)])
        .ok()?;
    Some(task)
}

pub fn edit_todo(db: &Connection, task: Task) -> Option<Task> {
    let db = get_db_connection(db)?;
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

pub fn get_todays_todos(db: &Connection) -> Option<Vec<Task>> {
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let today = NaiveDateTime::new(Utc::now().date_naive(), time).to_string();
    prepare_todos(
        db,
        &"SELECT * FROM tasks WHERE date_added >= ?1".to_string(),
        [today],
    )
}

pub fn get_all_todos(db: &Connection) -> Option<Vec<Task>> {
    prepare_todos(db, &"SELECT * FROM tasks".to_string(), [])
}

fn prepare_todos<P>(db: &Connection, query: &String, params: P) -> Option<Vec<Task>>
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

    match tasks.len() {
        0 => None,
        _ => Some(tasks),
    }
}

pub fn get_todo(db: &Connection, id: usize) -> Option<Task> {
    let db = get_db_connection(db)?;
    let mut statement = db.prepare("SELECT * FROM tasks WHERE id = ?1").ok()?;
    statement
        .query_row([id], |row| Ok(Task::from_db(row)))
        .ok()?
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{add_todo, edit_todo, get_todays_todos, remove_todo},
        types::{DbMemory, Task},
    };

    #[test]
    fn it_can_manage_todos() {
        let db = DbMemory::new().unwrap();

        let task_from_add = add_todo(&db, Task::new("I am Imogen!".to_string()));
        assert!(task_from_add.is_some());

        let tasks_from_today = get_todays_todos(&db);
        assert!(tasks_from_today.is_some());
        assert_eq!(tasks_from_today.unwrap().len(), 1);

        let mut task_for_edit = task_from_add.clone().unwrap();
        task_for_edit.description = "I am Adam!".to_string();
        let task_from_edit = edit_todo(&db, task_for_edit);
        assert!(task_from_edit.is_some());
        assert_eq!(task_from_edit.unwrap().description, "I am Adam!");

        let remove = remove_todo(&db, task_from_add.unwrap());
        assert!(remove.is_some());

        let todays_todos = get_todays_todos(&db);
        assert!(todays_todos.is_none());
    }
}
