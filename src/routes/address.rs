use super::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::db::CrmDbConn;
use crate::models::address::AddressEntity;
use crate::services::address;
use rocket_contrib::json::Json;

#[get("/address")]
pub fn find_all(token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = address::find_all(&conn);
    rocket_status_from_response(response)
}

#[get("/address/id/<id>")]
pub fn find_by_id(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = address::find_by_id(id, &conn);
    rocket_status_from_response(response)
}

#[post("/address", format = "json", data = "<address>")]
pub fn insert(
    address: Json<AddressEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = address::insert(&address.0, &conn);
    rocket_status_from_response(response)
}

#[put("/address/<id>", format = "json", data = "<address>")]
pub fn update(
    id: i32,
    address: Json<AddressEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = address::update(id, &address.0, &conn);
    rocket_status_from_response(response)
}

#[delete("/address/<id>")]
pub fn delete(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = address::delete(id, &conn);
    rocket_status_from_response(response)
}
