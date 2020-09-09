use crate::db::user::{insert_user, User};
use crate::CrmDbConn;
use rocket::{post, response::status};
use rocket_contrib::json::Json;

#[post("/users/create", format = "json", data = "<user>")]
pub fn create_user(
    conn: CrmDbConn,
    user: Json<User>,
) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let result = insert_user(&conn.0, user);
    match result {
        Ok(user) => Ok(status::Accepted(Some(format!(
            "Succesfully added user {} with id {} to the database",
            user.name, user.id
        )))),
        Err(err) => Err(status::BadRequest(Some(err.to_string()))),
    }
}
