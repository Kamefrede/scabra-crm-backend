use crate::constants::message_constants::{
    MESSAGE_LOGIN_FAILED, MESSAGE_LOGIN_SUCCESS, MESSAGE_SIGNUP_FAILED, MESSAGE_SIGNUP_SUCCESS,
};
use crate::db::user_auth_token::generate_jwt_token;
use crate::db::CrmDbConn;
use crate::models::response::ResponseWithStatus;
use crate::models::user::{LoginInfo, User, UserEntity};
use rocket::http::Status;

pub fn signup(user: LoginInfo, conn: &CrmDbConn) -> ResponseWithStatus {
    if User::signup(user, &**conn) {
        ResponseWithStatus::ok_message_without_data(String::from(MESSAGE_SIGNUP_SUCCESS))
    } else {
        ResponseWithStatus::status_message_without_data(
            String::from(MESSAGE_SIGNUP_FAILED),
            Status::BadRequest.code,
        )
    }
}

pub fn login(login: &LoginInfo, conn: &CrmDbConn) -> ResponseWithStatus {
    if let (Some(result), username) = User::login(login, &**conn) {
        ResponseWithStatus::ok_message_with_data(
            json!({ "token": generate_jwt_token(&result), "type": "Bearer", "username": username, "id": result.user_id }),
            String::from(MESSAGE_LOGIN_SUCCESS),
        )
    } else {
        ResponseWithStatus::status_message_without_data(
            String::from(MESSAGE_LOGIN_FAILED),
            Status::BadRequest.code,
        )
    }
}

pub fn update(id: i32, updated_user: &UserEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if User::get_user_by_id(id, &**conn).is_some() {
        if User::update(updated_user, id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_update()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("No user with id {} was found", id))
    }
}

pub fn delete(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if User::get_user_by_id(id, &**conn).is_some() {
        if User::delete(id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("No user with id {} was found", id))
    }
}
