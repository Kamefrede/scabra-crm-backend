pub mod address;
pub mod client;
pub mod employee;
pub mod person;
pub mod profile;
pub mod response;
pub mod user;
pub mod user_auth_token;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm)]
pub struct Query {
    pub query_type: String,
    pub query_text: String,
}
