use crate::models::user_auth_token::UserAuthToken;
use crate::models::response::{Response, ResponseWithStatus};
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

pub mod person;
pub mod user;
pub mod address;

pub type CustomJsonResponse = status::Custom<Json<Response>>;
pub type JsonWebToken = Result<UserAuthToken, CustomJsonResponse>;

pub fn rocket_status_from_response(response: ResponseWithStatus) -> CustomJsonResponse {
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
