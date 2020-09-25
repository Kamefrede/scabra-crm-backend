use crate::constants::message_constants::{
    MESSAGE_CAN_NOT_DELETE_DATA, MESSAGE_CAN_NOT_INSERT_DATA, MESSAGE_CAN_NOT_UPDATE_DATA,
    MESSAGE_OK,
};
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
    pub fn ok_empty() -> Self {
        Self::status_message_without_data(String::from(MESSAGE_OK), Status::Ok.code)
    }

    pub fn error_update() -> Self {
        Self::status_message_without_data(
            String::from(MESSAGE_CAN_NOT_UPDATE_DATA),
            Status::InternalServerError.code,
        )
    }

    pub fn error_insert() -> Self {
        Self::status_message_without_data(
            String::from(MESSAGE_CAN_NOT_INSERT_DATA),
            Status::InternalServerError.code,
        )
    }

    pub fn error_delete() -> Self {
        Self::status_message_without_data(
            String::from(MESSAGE_CAN_NOT_DELETE_DATA),
            Status::InternalServerError.code,
        )
    }

    pub fn eror_not_found(message: String) -> Self {
        Self::status_message_without_data(message, Status::NotFound.code)
    }

    pub fn status_message_with_data<T>(value: T, message: String, status_code: u16) -> Self
    where
        T: Serialize,
    {
        Self {
            status_code,
            response: Response {
                message,
                data: serde_json::to_value(value).unwrap(),
            },
        }
    }

    pub fn status_message_without_data(message: String, status_code: u16) -> Self {
        Self::status_message_with_data("", message, status_code)
    }

    pub fn ok_message_without_data(message: String) -> Self {
        Self::status_message_without_data(message, Status::Ok.code)
    }

    pub fn ok_message_with_data<T>(value: T, message: String) -> Self
    where
        T: Serialize,
    {
        Self::status_message_with_data(value, message, Status::Ok.code)
    }

    pub fn ok_with_data<T>(value: T) -> Self
    where
        T: Serialize,
    {
        Self::ok_message_with_data(value, String::from(MESSAGE_OK))
    }
}
