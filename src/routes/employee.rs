use super::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::db::CrmDbConn;
use crate::models::employee::Employee;
use crate::services::employee;
use rocket_contrib::json::Json;

#[get("/employee")]
pub fn find_all(token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = employee::find_all(&conn);
    rocket_status_from_response(response)
}

#[get("/employee/id/<id>")]
pub fn find_by_id(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = employee::find_by_id(id, &conn);
    rocket_status_from_response(response)
}

#[get("/employee/company/<id>")]
pub fn find_all_employees_by_company(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = employee::find_all_employees_by_company(id, &conn);
    rocket_status_from_response(response)
}

#[post("/employee", format = "json", data = "<employee>")]
pub fn insert(employee: Json<Employee>, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = employee::insert(&employee.0, &conn);
    rocket_status_from_response(response)
}

#[put("/employee/<id>", format = "json", data = "<employee>")]
pub fn update(
    id: i32,
    employee: Json<Employee>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = employee::update( &employee.0,id, &conn);
    rocket_status_from_response(response)
}

#[delete("/employee/<id>")]
pub fn delete(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = employee::delete(id, &conn);
    rocket_status_from_response(response)
}
