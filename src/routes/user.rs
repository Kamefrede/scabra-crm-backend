use super::{rocket_status_from_response, CustomJsonResponse};
use crate::db::CrmDbConn;
use crate::models::user::{LoginInfo, UserEntity};
use crate::routes::JsonWebToken;
use crate::services::user;
use rocket::post;
use rocket_contrib::json::Json;

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<LoginInfo>, conn: CrmDbConn, token: JsonWebToken) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = user::signup(user.0, &conn);
    rocket_status_from_response(response)
}

#[post("/login", format = "application/json", data = "<login>")]
pub fn login(login: Json<LoginInfo>, conn: CrmDbConn) -> CustomJsonResponse {
    let response = user::login(&login.0, &conn);
    rocket_status_from_response(response)
}

#[put("/user/<id>", format = "json", data = "<user>")]
pub fn update(
    id: i32,
    user: Json<UserEntity>,
    token: JsonWebToken,
    conn: CrmDbConn,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = user::update(id, &user.0, &conn);
    rocket_status_from_response(response)
}

#[delete("/user/<id>")]
pub fn delete(id: i32, token: JsonWebToken, conn: CrmDbConn) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = user::delete(id, &conn);
    rocket_status_from_response(response)
}
