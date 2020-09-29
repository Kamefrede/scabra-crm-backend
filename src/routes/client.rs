use super::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::db::CrmDbConn;
use crate::models::client::ClientEntity;
use crate::models::Query;
use crate::services::client;
use rocket_contrib::json::Json;

#[get("/client")]
pub fn find_all(token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = client::find_all(&conn);
    rocket_status_from_response(response)
}

#[get("/client/id/<id>")]
pub fn find_by_id(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = client::find_by_id(id, &conn);
    rocket_status_from_response(response)
}

#[post("/client/query", format = "json", data = "<query>")]
pub fn query(query: Json<Query>, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = client::query(query.0, &conn);
    rocket_status_from_response(response)
}

#[post("/client", format = "json", data = "<client>")]
pub fn insert(
    client: Json<ClientEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = client::insert(&client.0, &conn);
    rocket_status_from_response(response)
}

#[put("/client/<id>", format = "json", data = "<client>")]
pub fn update(
    id: i32,
    client: Json<ClientEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = client::update(id, &client.0, &conn);
    rocket_status_from_response(response)
}

#[delete("/client/<id>")]
pub fn delete(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e
    }
    let response = client::delete(id, &conn);
    rocket_status_from_response(response)
}
