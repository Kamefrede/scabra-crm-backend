use crate::constants::message_constants::MESSAGE_OK;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}

impl ResponseWithStatus {
    pub fn ok_empty() -> ResponseWithStatus {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }

    pub fn ok_with_data<T>(value: T, message: String) -> ResponseWithStatus
    where
        T: Serialize,
    {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message,
                data: serde_json::to_value(value).unwrap(),
            },
        }
    }
}
