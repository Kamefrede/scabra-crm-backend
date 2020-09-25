use std::env;
use std::ops::Deref;

use chrono::Utc;
use crypto::digest::Digest;
use crypto::sha3;

use crate::models::user::User;
use crate::models::user::UserForm;
use crate::models::user_auth_token::{LoginInfo, UserAuthToken};
use crate::schema::user::dsl::{email, id, user, username};
use crate::schema::user_auth_token::dsl::{
    expires_at, generated_at, login_session, user_auth_token, user_id,
};
use diesel::prelude::*;
use uuid::Uuid;

impl User {
    pub fn signup(user_form: UserForm, conn: &PgConnection) -> bool {
        let hashed_pwd = hash_password(&*user_form.hashed_password);
        let user_form = UserForm {
            hashed_password: hashed_pwd,
            ..user_form
        };
        user.filter(username.eq(&*user_form.username))
            .or_filter(email.eq(&*user_form.email))
            .get_result::<Self>(conn)
            .map_or(
                diesel::insert_into(user)
                    .values(&user_form)
                    .execute(conn)
                    .is_ok(),
                |_existing_user| false,
            )
    }

    pub fn login(user_form: &LoginInfo, conn: &PgConnection) -> Option<UserAuthToken> {
        Self::get_user_by_username_or_email(&*user_form.username_or_email, conn).and_then(
            |db_user| {
                if db_user.hashed_password.is_empty()
                    && !verify_password(&*user_form.password, &*db_user.hashed_password)
                {
                    return None;
                }
                let login_session_str = Self::generate_login_session();
                let auth_token =
                    UserAuthToken::generate_auth_token(db_user.id, &*login_session_str);
                Self::update_login_session(&auth_token, conn);
                Some(auth_token)
            },
        )
    }

    pub fn get_user_by_id(id_user: i32, conn: &PgConnection) -> Option<Self> {
        user.filter(id.eq(&id_user)).get_result::<Self>(conn).ok()
    }

    pub fn get_user_by_username_or_email(
        username_or_email: &str,
        conn: &PgConnection,
    ) -> Option<Self> {
        user.filter(username.eq(username_or_email))
            .or_filter(email.eq(username_or_email))
            .get_result::<Self>(conn)
            .ok()
    }
    pub fn get_id_for_username_or_email(
        username_or_email: &str,
        conn: &PgConnection,
    ) -> Option<i32> {
        Self::get_user_by_username_or_email(username_or_email, conn).map(|db_user| db_user.id)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn insert_login_session(
        auth_token: &UserAuthToken,
        conn: &PgConnection,
    ) -> Result<UserAuthToken, diesel::result::Error> {
        diesel::insert_into(user_auth_token)
            .values(auth_token)
            .get_result(conn)
    }

    pub fn is_valid_login_session(auth_token: &UserAuthToken, conn: &PgConnection) -> bool {
        user_auth_token
            .filter(user_id.eq(&auth_token.user_id))
            .filter(login_session.eq(&auth_token.login_session))
            .get_result::<UserAuthToken>(conn)
            .map_or(false, |auth_token: UserAuthToken| {
                Utc::now().naive_utc() < *auth_token.expires_at.deref()
            })
    }

    pub fn update_login_session(new_auth_token: &UserAuthToken, conn: &PgConnection) -> bool {
        if let Some(_user) = Self::get_user_by_id(new_auth_token.user_id, conn) {
            if let Err(_e) = user_auth_token
                .find(new_auth_token.user_id)
                .get_result::<UserAuthToken>(conn)
            {
                !matches!(Self::insert_login_session(new_auth_token, conn), Err(_e))
            } else {
                diesel::update(user_auth_token.find(new_auth_token.user_id))
                    .set((
                        user_id.eq(new_auth_token.user_id),
                        login_session.eq(new_auth_token.login_session.clone()),
                        generated_at.eq(new_auth_token.generated_at),
                        expires_at.eq(new_auth_token.expires_at),
                    ))
                    .execute(conn)
                    .is_ok()
            }
        } else {
            false
        }
    }
}

fn hash_password(password: &str) -> String {
    let mut hasher = sha3::Sha3::sha3_256();
    let mut salt = env::var("SALT").expect("SALT key must be present in .env file");
    salt.push_str(password);
    hasher.input_str(&salt);
    hasher.result_str()
}

fn verify_password(password_to_verify: &str, password: &str) -> bool {
    password == hash_password(password_to_verify)
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
