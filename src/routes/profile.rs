use super::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::db::CrmDbConn;
use crate::models::profile::Profile;
use crate::models::Query;
use crate::services::profile;
use rocket_contrib::json::Json;

#[get("/profile")]
pub fn find_all(token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = profile::find_all(&conn);
    rocket_status_from_response(response)
}

#[get("/profile/id/<id>")]
pub fn find_by_id(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = profile::find_by_id(id, &conn);
    rocket_status_from_response(response)
}

#[post("/profile/query", format = "json", data = "<query>")]
pub fn query(query: Json<Query>, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = profile::query(query.0, &conn);
    rocket_status_from_response(response)
}

#[post("/profile", format = "json", data = "<profile>")]
pub fn insert(profile: Json<Profile>, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = profile::insert(&profile.0, &conn);
    rocket_status_from_response(response)
}

#[put("/profile/<id>", format = "json", data = "<profile>")]
pub fn update(
    id: i32,
    profile: Json<Profile>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = profile::update(id, &profile.0, &conn);
    rocket_status_from_response(response)
}

#[delete("/profile/<id>")]
pub fn delete(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = profile::delete(id, &conn);
    rocket_status_from_response(response)
}
