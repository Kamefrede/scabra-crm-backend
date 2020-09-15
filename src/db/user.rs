use crate::db::user_auth_token::UserAuthToken;
use crate::models::user::UserEntity;
use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::routes::user::UserForm;
use crate::schema::*;
use crypto::digest::Digest;
use crypto::sha3;
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "user"]
pub struct User {
    pub username: String,
    pub person_id: Option<i32>,
    pub email: String,
    pub hashed_password: String,
}

impl User {
    pub fn signup(
        user_form: &UserForm,
        conn: &PgConnection,
    ) -> Result<UserEntity, diesel::result::Error> {
        let user: User = User {
            username: user_form.username.clone(),
            person_id: None,
            email: user_form.email.clone(),
            hashed_password: hash_password(&user_form.password),
        };
        diesel::insert_into(user::table)
            .values(&user)
            .get_result(conn)
    }

    pub fn login(
        user_form: &UserForm,
        conn: &PgConnection,
    ) -> Result<UserAuthToken, diesel::result::Error> {
        unimplemented!()
    }

    pub fn get_user_by_id(
        id: i32,
        conn: &PgConnection,
    ) -> Result<UserEntity, diesel::result::Error> {
        use crate::schema::user::dsl::*;
        user.filter(id.eq(&id)).get_result::<UserEntity>(conn)
    }
    pub fn get_user_by_username_or_email(
        username_or_email: &str,
        conn: &PgConnection,
    ) -> Result<UserEntity, diesel::result::Error> {
        use crate::schema::user::dsl::*;
        user.filter(username.eq(username_or_email))
            .or_filter(email.eq(username_or_email))
            .get_result::<UserEntity>(conn)
    }
    pub fn get_id_for_username_or_email(
        username_or_email: &str,
        conn: &PgConnection,
    ) -> Option<i32> {
        let id: Option<i32> = match User::get_user_by_username_or_email(username_or_email, conn) {
            Ok(user) => Some(user.id),
            Err(_) => None,
        };
        id
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn insert_login_session(
        auth_token: &UserAuthToken,
        conn: &PgConnection,
    ) -> Result<UserAuthToken, diesel::result::Error> {
        diesel::insert_into(user_auth_token::table)
            .values(auth_token)
            .get_result(conn)
    }

    pub fn is_valid_login_session(auth_token: &UserAuthToken, conn: &PgConnection) -> bool {
        use crate::schema::user_auth_token::dsl::*;
        user_auth_token
            .filter(user_id.eq(&auth_token.user_id))
            .filter(login_session.eq(&auth_token.login_session))
            .get_result::<UserAuthToken>(conn)
            .is_ok()

        //TODO: Check if date is less than current date
    }

    pub fn update_login_session(
        user_id: i32,
        login_session: &str,
        generated_at: &NaiveDateForm,
        expires_at: &NaiveDateForm,
        conn: &PgConnection,
    ) {
        unimplemented!()
    }
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
    use dotenv::dotenv;
    dotenv().ok();
    let correct_hash =
        env::var("CORRECT_HASH").expect("CORRECT_HASH key must be present in .env file");
    let string_to_hash: String =
        env::var("STRING_TO_HASH").expect("STRING_TO_HASH key must be present in .env file");
    assert_eq!(hash_password(&string_to_hash), correct_hash)
}
