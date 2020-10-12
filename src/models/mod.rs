pub mod address;
pub mod calendar;
pub mod client;
pub mod person;
pub mod response;
pub mod task;
pub mod user;
pub mod user_auth_token;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm)]
pub struct Query {
    pub query_type: String,
    pub query_text: String,
}
