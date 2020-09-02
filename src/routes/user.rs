use crate::CrmDbConn;
use crate::db::user::{User, insert_user};
use rocket_contrib::json::{Json};
use rocket::{post, response::{status}};

#[post("/users/create", format = "json", data = "<user>")]
pub fn create_user(conn: CrmDbConn, user: Json<User>) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let result = insert_user(&conn.0, user);
    match result {
        Ok(user) => {
            Ok(status::Accepted(Some(format!("Succesfully added user {} with id {} to the database", user.name, user.id))))
        }
        Err(err) => {
            Err(status::BadRequest(Some(err.to_string())))
        }
    }
}