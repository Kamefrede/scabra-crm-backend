use super::{rocket_status_from_response, CustomJsonResponse};
use crate::db::CrmDbConn;
use crate::schema::user;
use crate::services;
use rocket::post;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Insertable, FromForm, Serialize, Deserialize)]
#[table_name = "user"]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct UserLoginInfo {
    pub username_or_email: String,
    pub password: String,
}

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<UserForm>, conn: CrmDbConn) -> CustomJsonResponse {
    let response = services::user::signup(user.0, conn);
    rocket_status_from_response(response)
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<UserLoginInfo>, conn: CrmDbConn) -> CustomJsonResponse {
    let response = services::user::login(login.0, conn);
    rocket_status_from_response(response)
}
