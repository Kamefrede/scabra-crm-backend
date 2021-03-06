use crate::db::CrmDbConn;
use crate::models::calendar::CalendarState;
use crate::models::response::ResponseWithStatus;
use crate::models::task::{Task, TaskEntity};
use rocket::State;

pub fn find_all(conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Task::find_all(&**conn))
}

pub fn find_by_id(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_task = Task::find_by_id(id, &**conn);
    if let Some(task) = option_task {
        ResponseWithStatus::ok_with_data(task)
    } else {
        ResponseWithStatus::error_not_found(format!("Task with id {} was not found", id))
    }
}

pub fn find_all_user_tasks(user_id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Task::find_all_user_tasks(user_id, &**conn))
}

pub fn find_all_cient_tasks(client_id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Task::find_all_client_tasks(client_id, &**conn))
}

pub fn insert(
    new_task: &TaskEntity,
    conn: &CrmDbConn,
    state: &State<CalendarState>,
) -> ResponseWithStatus {
    if Task::insert(new_task, &**conn, state) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_insert()
    }
}

pub fn update(
    updated_task: &TaskEntity,
    task_id: i32,
    conn: &CrmDbConn,
    state: &State<CalendarState>,
) -> ResponseWithStatus {
    if Task::find_by_id(task_id, &**conn).is_some() {
        if Task::update(updated_task, task_id, &**conn, state) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_update()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("Couldn't find task with id {}", task_id))
    }
}

pub fn delete(task_id: i32, conn: &CrmDbConn, state: &State<CalendarState>) -> ResponseWithStatus {
    if Task::find_by_id(task_id, &**conn).is_some() {
        if Task::delete(task_id, &**conn, state) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("Couldn't find task with id {}", task_id))
    }
}
