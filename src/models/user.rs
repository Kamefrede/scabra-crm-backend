use crate::schema::user;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Identifiable, Queryable, Deserialize, Serialize)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub person_id: Option<i32>,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Insertable, FromForm, Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserEntity {
    pub person_id: Option<i32>,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Insertable, FromForm, Serialize, Deserialize)]
#[table_name = "user"]
pub struct LoginInfo {
    pub email: String,
    pub hashed_password: String,
}
