use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::schema::user_auth_token;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "user_auth_token"]
#[primary_key(user_id)]
pub struct UserAuthToken {
    pub user_id: i32,
    pub login_session: String,
    pub generated_at: NaiveDateForm,
    pub expires_at: NaiveDateForm,
}

pub const ONE_WEEK: i64 = 60 * 60 * 24 * 7;
