use crate::db::CrmDbConn;
use crate::models::response::Response;
use crate::schema::user;
use crate::services;
use diesel::prelude::Insertable;
use rocket::http::Status;
use rocket::{post, response::status};
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
pub fn signup(user: Json<UserForm>, conn: CrmDbConn) -> status::Custom<Json<Response>> {
    let response = services::user::signup(user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<UserLoginInfo>, conn: CrmDbConn) -> status::Custom<Json<Response>> {
    let response = services::user::login(login.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
