use crate::db::user::User;
use crate::CrmDbConn;
use rocket::{post, response::status};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(FromForm, Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/users/create", format = "json", data = "<form_user>")]
pub fn create_user(
    conn: CrmDbConn,
    form_user: Json<UserForm>,
) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let result = User::signup(&form_user.0, &conn.0);
    match result {
        Ok(user) => Ok(status::Accepted(Some(format!(
            "Successfully added user {} with id {} to the database",
            user.username, user.id
        )))),
        Err(err) => Err(status::BadRequest(Some(err.to_string()))),
    }
}
