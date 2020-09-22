use std::env;
use std::ops::Deref;

use chrono::Utc;
use crypto::digest::Digest;
use crypto::sha3;

use uuid::Uuid;
use crate::models::user::User;
use diesel::prelude::*;
use crate::models::user_auth_token::{UserAuthToken, LoginInfo};
use crate::models::user::UserForm;
use crate::schema::user_auth_token;

impl User {
    pub fn signup(user_form: UserForm, conn: &PgConnection) -> bool {
        let hashed_pwd = hash_password(&*user_form.hashed_password);
        let user_form = UserForm {
            hashed_password: hashed_pwd,
            ..user_form
        };
        use crate::schema::user::dsl::*;
        let already_exists = user
            .filter(username.eq(&*user_form.username))
            .or_filter(email.eq(&*user_form.email))
            .get_result::<User>(conn);
        if let Ok(_existing_user) = already_exists {
            false
        } else {
            diesel::insert_into(user)
                .values(&user_form)
                .execute(conn)
                .is_ok()
        }
    }

    pub fn login(user_form: LoginInfo, conn: &PgConnection) -> Option<UserAuthToken> {
        let result_user = Self::get_user_by_username_or_email(&*user_form.username_or_email, conn);
        if let Some(user) = result_user {
            if !user.hashed_password.is_empty()
                && verify_password(&*user_form.password, &*user.hashed_password)
            {
                let login_session_str = Self::generate_login_session();
                let auth_token = UserAuthToken::generate_auth_token(user.id, &*login_session_str);
                Self::update_login_session(&auth_token, conn);
                Some(auth_token)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_user_by_id(user_id: i32, conn: &PgConnection) -> Option<User> {
        use crate::schema::user::dsl::*;
        let result_user = user.filter(id.eq(&user_id)).get_result::<User>(conn);
        if let Ok(db_user) = result_user {
            Some(db_user)
        } else {
            None
        }
    }

    pub fn get_user_by_username_or_email(
        username_or_email: &str,
        conn: &PgConnection,
    ) -> Option<User> {
        use crate::schema::user::dsl::*;
        let result_user = user
            .filter(username.eq(username_or_email))
            .or_filter(email.eq(username_or_email))
            .get_result::<User>(conn);
        if let Ok(db_user) = result_user {
            Some(db_user)
        } else {
            None
        }
    }
    pub fn get_id_for_username_or_email(
        username_or_email: &str,
        conn: &PgConnection,
    ) -> Option<i32> {
        if let Some(user) = User::get_user_by_username_or_email(username_or_email, conn) {
            Some(user.id)
        } else {
            None
        }
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
        let result_auth_token: Result<UserAuthToken, diesel::result::Error> = user_auth_token
            .filter(user_id.eq(&auth_token.user_id))
            .filter(login_session.eq(&auth_token.login_session))
            .get_result::<UserAuthToken>(conn);
        if let Ok(auth_token) = result_auth_token {
            Utc::now().naive_utc() < *auth_token.expires_at.deref()
        } else {
            false
        }
    }

    pub fn update_login_session(new_auth_token: &UserAuthToken, conn: &PgConnection) -> bool {
        use crate::schema::user_auth_token::dsl::*;
        if let Some(_user) = User::get_user_by_id(new_auth_token.user_id, conn) {
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

fn verify_password(password_to_verify: &str, hashed_password: &str) -> bool {
    hashed_password == hash_password(password_to_verify)
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
