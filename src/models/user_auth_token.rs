use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::schema::user_auth_token;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "user_auth_token"]
pub struct UserAuthToken {
    pub user_id: i32,
    pub login_session: String,
    pub generated_at: NaiveDateForm,
    pub expires_at: NaiveDateForm,
}

pub const ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginInfo {
    pub username_or_email: String,
    pub password: String,
}
