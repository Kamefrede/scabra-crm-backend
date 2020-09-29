#![allow(clippy::needless_pass_by_value)]
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user_auth_token::UserAuthToken;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

pub mod address;
pub mod client;
pub mod employee;
pub mod person;
pub mod profile;
pub mod user;

pub type CustomJsonResponse = status::Custom<Json<Response>>;
pub type JsonWebToken = Result<UserAuthToken, CustomJsonResponse>;

pub fn rocket_status_from_response(response: ResponseWithStatus) -> CustomJsonResponse {
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
