use crate::models::user::UserEntity;
use crate::schema::*;
use crypto::digest::Digest;
use crypto::sha3;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name = "user"]
pub struct User {
    pub username: String,
    pub person_id: Option<i32>,
    pub email: String,
    pub hashed_password: String,
}

pub fn insert_user(
    conn: &PgConnection,
    form_user: &Json<User>,
) -> Result<UserEntity, diesel::result::Error> {
    let u = User {
        username: form_user.username.clone(),
        person_id: None, //TODO: at some point implement this
        email: form_user.email.clone(),
        hashed_password: hash_password(&form_user.hashed_password),
    };
    diesel::insert_into(user::table).values(&u).get_result(conn)
}

fn hash_password(password: &str) -> String {
    let mut hasher = sha3::Sha3::sha3_256();
    let mut salt = env::var("SALT").expect("SALT key must be present in .env file");
    salt.push_str(password);
    hasher.input_str(&salt);
    hasher.result_str()
}

#[test]
fn test_hash() {
    dotenv().ok();
    let correct_hash =
        env::var("CORRECT_HASH").expect("CORRECT_HASH key must be present in .env file");
    assert_eq!(hash_password(&"".to_string()), correct_hash)
}
