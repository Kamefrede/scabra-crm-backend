use crate::constants::message_constants;
use crate::models::user::User;
use crate::db::user_auth_token::generate_jwt_token;
use crate::db::CrmDbConn;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::UserForm;
use crate::models::user_auth_token::LoginInfo;
use rocket::http::Status;

pub fn signup(user: UserForm, conn: CrmDbConn) -> ResponseWithStatus {
    if User::signup(user, &*conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn login(login: LoginInfo, conn: CrmDbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &*conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_SUCCESS),
                data: serde_json::to_value(
                    json!({ "token": generate_jwt_token(&result), "type": "Bearer" }),
                )
                .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
