use super::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::db::CrmDbConn;
use crate::models::person::PersonEntity;
use crate::services::person;
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[get("/person")]
pub fn find_all(token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = person::find_all(&conn);
    rocket_status_from_response(response)
}

#[get("/person/id/<id>")]
pub fn find_by_id(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = person::find_by_id(id, &conn);
    rocket_status_from_response(response)
}

#[get("/person/name/<name>")]
pub fn find_by_name(name: &RawStr, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = person::find_by_name(name, &conn);
    rocket_status_from_response(response)
}

#[post("/person", format = "json", data = "<person>")]
pub fn insert(
    person: Json<PersonEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = person::insert(&person.0, &conn);
    rocket_status_from_response(response)
}

#[put("/person/<id>", format = "json", data = "<person>")]
pub fn update(
    id: i32,
    person: Json<PersonEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = person::update(id, &person.0, &conn);
    rocket_status_from_response(response)
}

#[delete("/person/<id>")]
pub fn delete(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = person::delete(id, &conn);
    rocket_status_from_response(response)
}
