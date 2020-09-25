pub mod address;
pub mod person;
pub mod response;
pub mod user;
pub mod user_auth_token;
pub mod profile;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm)]
pub struct Query {
    pub query_type: String,
    pub query_text: String,
}
