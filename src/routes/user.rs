use super::{rocket_status_from_response, CustomJsonResponse};
use crate::db::CrmDbConn;
use crate::models::user::UserForm;
use crate::models::user_auth_token::LoginInfo;
use crate::services;
use rocket::post;
use rocket_contrib::json::Json;

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<UserForm>, conn: CrmDbConn) -> CustomJsonResponse {
    let response = services::user::signup(user.0, &conn);
    rocket_status_from_response(response)
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<LoginInfo>, conn: CrmDbConn) -> CustomJsonResponse {
    let response = services::user::login(&login.0, &conn);
    rocket_status_from_response(response)
}
