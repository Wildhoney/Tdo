use chrono::{Duration, NaiveDateTime, NaiveTime, Utc};
use rand::seq::SliceRandom;
use rusqlite::Connection;

mod utils;

use crate::types::{Task, TaskStatus, TodosFor};

use self::utils::{get_db_connection, prepare_todos};

pub fn add_todo(db: &Connection, task: Task) -> Option<Task> {
    let db = get_db_connection(db)?;
    let date_for = task.date_for.unwrap_or(Utc::now().naive_local());

    db.execute(
        "INSERT INTO tasks (description, status, date_for) VALUES (?1, ?2, ?3)",
        (
            &task.description,
            task.status.as_str(),
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
        "UPDATE tasks SET description = ?1, status = ?2, date_modified = CURRENT_TIMESTAMP WHERE id = ?3",
        (
            &task.description,
            task.status.as_str(),
            &task.id.unwrap_or(0),
        ),
    )
    .ok()?;
    Some(task)
}

pub fn extend_todo(db: &Connection, task: Task) -> Option<Task> {
    let db = get_db_connection(db)?;
    let date_for = task.date_for?.format("%Y-%m-%d %H:%M:%S").to_string();
    db.execute(
        "UPDATE tasks SET date_for = ?1, date_modified = CURRENT_TIMESTAMP WHERE id = ?2",
        (&date_for, &task.id.unwrap_or(0)),
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
                &"SELECT * FROM tasks WHERE status != ?1 AND date_for >= ?2 ORDER BY date_added ASC".to_string(),
                [TaskStatus::Done.as_str().to_string(), beginning_of_tomorrow],
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
                  OR    (date_for < ?3 AND status != ?4)
                  OR    (date_modified >= ?1 AND date_modified < ?2)
                  ORDER BY date_added ASC"
                    .to_string(),
                [
                    start_of_today,
                    beginning_of_tomorrow,
                    beginning_of_today,
                    TaskStatus::Done.as_str().to_string(),
                ],
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
            .filter(|t| t.status != TaskStatus::Done)
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .cloned(),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use crate::{
        db::{add_todo, edit_todo, extend_todo, get_random_todo, get_todos, remove_todo},
        types::{DbMemory, Task, TaskStatus, TodosFor},
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

    #[test]
    fn it_can_randomly_find_tasks() {
        let db = DbMemory::new().unwrap();

        let random_task = get_random_todo(&db);
        assert_eq!(random_task.is_none(), true);

        add_todo(&db, Task::new("Buy a cup!".to_string(), None));
        add_todo(&db, Task::new("Buy a plate!".to_string(), None));
        add_todo(&db, Task::new("Buy a spoon!".to_string(), None));

        let random_task = get_random_todo(&db);
        assert_eq!(random_task.is_some(), true);
        assert_ne!(random_task.unwrap().status, TaskStatus::Done);
    }

    #[test]
    fn it_persists_in_progress_status() {
        let db = DbMemory::new().unwrap();

        let mut task = add_todo(&db, Task::new("Write the report".to_string(), None)).unwrap();
        assert_eq!(task.status, TaskStatus::Todo);

        task.status = TaskStatus::InProgress;
        let edited = edit_todo(&db, task).unwrap();
        assert_eq!(edited.status, TaskStatus::InProgress);

        let todos = get_todos(TodosFor::Today, &db).unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].status, TaskStatus::InProgress);
    }

    #[test]
    fn it_includes_in_progress_in_random_picker_and_excludes_done() {
        let db = DbMemory::new().unwrap();

        let mut a = add_todo(&db, Task::new("Task A".to_string(), None)).unwrap();
        let mut b = add_todo(&db, Task::new("Task B".to_string(), None)).unwrap();

        a.status = TaskStatus::InProgress;
        b.status = TaskStatus::Done;
        edit_todo(&db, a).unwrap();
        edit_todo(&db, b).unwrap();

        for _ in 0..20 {
            let picked = get_random_todo(&db).unwrap();
            assert_eq!(picked.description, "Task A");
            assert_eq!(picked.status, TaskStatus::InProgress);
        }
    }

    #[test]
    fn it_extends_a_task_by_24_hours() {
        let db = DbMemory::new().unwrap();

        let yesterday = Utc::now().naive_local() - Duration::days(1);
        let task = Task::new("Overdue task".to_string(), Some(yesterday));
        let added = add_todo(&db, task).unwrap();
        let original = added.date_for.unwrap();

        let mut extended = added.clone();
        extended.date_for = Some(original + Duration::hours(24));
        let extended = extend_todo(&db, extended).unwrap();

        assert_eq!(extended.date_for.unwrap(), original + Duration::hours(24));
        assert_eq!(extended.description, added.description);
    }

    #[test]
    fn it_transitions_status_through_all_states() {
        let db = DbMemory::new().unwrap();

        let mut task = add_todo(&db, Task::new("Cycle me".to_string(), None)).unwrap();

        task.status = TaskStatus::InProgress;
        assert_eq!(
            edit_todo(&db, task.clone()).unwrap().status,
            TaskStatus::InProgress
        );

        task.status = TaskStatus::Done;
        assert_eq!(
            edit_todo(&db, task.clone()).unwrap().status,
            TaskStatus::Done
        );

        task.status = TaskStatus::Todo;
        assert_eq!(edit_todo(&db, task).unwrap().status, TaskStatus::Todo);
    }
}
