use crate::db::user::User;
use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::schema::user_auth_token;
use chrono::{NaiveDateTime, Timelike, Utc};
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "user_auth_token"]
pub struct UserAuthToken {
    pub user_id: i32,
    pub login_session: String,
    pub generated_at: NaiveDateForm,
    pub expires_at: NaiveDateForm,
}

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginInfo {
    username_or_email: String,
    password: String,
}
pub fn generate_auth_token(user_id: i32, login_session: &str) -> UserAuthToken {
    let now = Utc::now().naive_utc();
    let expiry_secs = now.second() as i64 + ONE_WEEK;
    let expiry = NaiveDateTime::from_timestamp(expiry_secs, 0);
    UserAuthToken {
        user_id,
        login_session: login_session.to_string(),
        generated_at: NaiveDateForm::new(now),
        expires_at: NaiveDateForm::new(expiry),
    }
}

pub fn generate_jwt_token(auth_token: &UserAuthToken) {
    unimplemented!()
}
