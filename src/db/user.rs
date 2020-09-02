use serde::{Serialize, Deserialize};
use crate::schema::*;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use rocket_contrib::json::Json;
use crate::models::user::UserEntity;

#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name="users"]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}


pub fn insert_user(conn: &PgConnection, form_user: Json<User>) -> Result<UserEntity, diesel::result::Error> {
    let u = User {
        name: form_user.name.clone(),
        email: form_user.email.clone(),
        age: form_user.age
    };
    diesel::insert_into(users::table)
        .values(&u)
        .get_result(conn)
}