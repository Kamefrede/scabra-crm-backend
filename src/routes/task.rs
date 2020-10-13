use super::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::db::CrmDbConn;
use crate::models::calendar::CalendarState;
use crate::models::task::TaskEntity;
use crate::models::Query;
use crate::services::task;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/task")]
pub fn find_all(token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::find_all(&conn);
    rocket_status_from_response(response)
}

#[get("/task/id/<id>")]
pub fn find_by_id(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::find_by_id(id, &conn);
    rocket_status_from_response(response)
}

#[get("/task/client/<id>")]
pub fn find_all_client_tasks(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::find_all_cient_tasks(id, &conn);
    rocket_status_from_response(response)
}

#[get("/task/user/<id>")]
pub fn find_all_user_tasks(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::find_all_user_tasks(id, &conn);
    rocket_status_from_response(response)
}

//TODO: Implement fuzzy searching
#[post("/task/query", format = "json", data = "<query>")]
pub fn query(query: Json<Query>, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::query(&query.0, &conn);
    rocket_status_from_response(response)
}

#[post("/task", format = "json", data = "<task>")]
pub fn insert(
    task: Json<TaskEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::insert(&task.0, &conn, &calendar_state);
    rocket_status_from_response(response)
}

#[put("/task/<id>", format = "json", data = "<task>")]
pub fn update(
    id: i32,
    task: Json<TaskEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::update(&task.0, id, &conn, &calendar_state);
    rocket_status_from_response(response)
}

#[delete("/task/<id>")]
pub fn delete(
    id: i32,
    token: JsonWebToken,
    conn: CrmDbConn,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = task::delete(id, &conn, &calendar_state);
    rocket_status_from_response(response)
}
