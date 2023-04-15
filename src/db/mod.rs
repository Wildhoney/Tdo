use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use rand::seq::SliceRandom;
use rusqlite::Connection;

mod utils;

use crate::types::{Task, TodosFor};

use self::utils::{get_db_connection, prepare_todos};

pub fn add_todo(db: &Connection, task: Task) -> Option<Task> {
    let db = get_db_connection(db)?;
    let date_for = task.date_for.unwrap_or(Utc::now().naive_local());

    db.execute(
        "INSERT INTO tasks (description, completed, date_for) VALUES (?1, ?2, ?3)",
        (
            &task.description,
            &task.completed,
            &date_for.format("%Y-%m-%d %H:%M:%S").to_string(),
        ),
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

pub fn get_todos(when: TodosFor, db: &Connection) -> Option<Vec<Task>> {
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();

    match when {
        TodosFor::Upcoming => {
            let beginning_of_tomorrow =
                NaiveDateTime::new(Utc::now().date_naive() + Duration::days(1), time).to_string();

            prepare_todos(
                db,
                &"SELECT * FROM tasks WHERE completed = 0 AND date_for >= ?1".to_string(),
                [beginning_of_tomorrow],
            )
        }
        TodosFor::Today => {
            let start_of_today = NaiveDateTime::new(Utc::now().date_naive(), time).to_string();
            let beginning_of_today =
                NaiveDateTime::new(Utc::now().date_naive() + Duration::days(1), time).to_string();
            let beginning_of_tomorrow =
                NaiveDateTime::new(Utc::now().date_naive() + Duration::days(1), time).to_string();

            prepare_todos(
                db,
                &"SELECT * FROM tasks
                  WHERE (date_for >= ?1 AND date_for < ?2)
                  OR    (date_for < ?3 AND completed = 0)
                  OR    (date_modified >= ?1 AND date_modified < ?2)
                  ORDER BY date_for DESC"
                    .to_string(),
                [start_of_today, beginning_of_tomorrow, beginning_of_today],
            )
        }
    }
}

pub fn get_todo(db: &Connection, id: usize) -> Option<Task> {
    let db = get_db_connection(db)?;
    let mut statement = db.prepare("SELECT * FROM tasks WHERE id = ?1").ok()?;
    statement
        .query_row([id], |row| Ok(Task::from_db(row)))
        .ok()?
}

pub fn get_random_todo(db: &Connection) -> Option<Task> {
    let todos = get_todos(TodosFor::Today, db);

    match todos {
        Some(todos) => todos
            .into_iter()
            .filter(|t| !t.completed)
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .cloned(),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{add_todo, edit_todo, get_todos, remove_todo},
        types::{DbMemory, Task, TodosFor},
    };

    #[test]
    fn it_can_manage_todos() {
        let db = DbMemory::new().unwrap();

        let task_from_add = add_todo(&db, Task::new("I am Imogen!".to_string(), None));
        assert!(task_from_add.is_some());

        let tasks_from_today = get_todos(TodosFor::Today, &db);
        assert!(tasks_from_today.is_some());
        assert_eq!(tasks_from_today.unwrap().len(), 1);

        let mut task_for_edit = task_from_add.clone().unwrap();
        task_for_edit.description = "I am Adam!".to_string();
        let task_from_edit = edit_todo(&db, task_for_edit);
        assert!(task_from_edit.is_some());
        assert_eq!(task_from_edit.unwrap().description, "I am Adam!");

        let remove = remove_todo(&db, task_from_add.unwrap());
        assert!(remove.is_some());
    }
}
