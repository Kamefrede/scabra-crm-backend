use crate::constants::message_constants::*;
use crate::db::CrmDbConn;
use crate::models::profile::Profile;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::Query;
use rocket::http::Status;

pub fn find_all(conn: CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(MESSAGE_OK),
            data: serde_json::to_value(Profile::find_all(&*conn)).unwrap(),
        },
    }
}

pub fn find_by_id(id: i32, conn: CrmDbConn) -> ResponseWithStatus {
    let option_profile = Profile::find_by_id(id, &*conn);
    if let Some(profile) = option_profile {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(MESSAGE_OK),
                data: serde_json::to_value(profile).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("Could not find profile with id {}", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn insert(profile: Profile, conn: CrmDbConn) -> ResponseWithStatus {
    if Profile::insert(profile, &*conn) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus {
            status_code: Status::InternalServerError.code,
            response: Response {
                message: String::from(MESSAGE_CAN_NOT_INSERT_DATA),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn query(query: Query, conn: CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Profile::query(query, &*conn), String::from(MESSAGE_OK))
}

pub fn update(id: i32, new_profile: Profile, conn: CrmDbConn) -> ResponseWithStatus {
    let option_profile = Profile::find_by_id(id, &*conn);
    if option_profile.is_some() {
        if Profile::update(id, new_profile, &*conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus {
                status_code: Status::InternalServerError.code,
                response: Response {
                    message: String::from(MESSAGE_CAN_NOT_UPDATE_DATA),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("Could not find profile with id {}", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn delete(id: i32, conn: CrmDbConn) -> ResponseWithStatus {
    let option_profile = Profile::find_by_id(id, &*conn);
    if option_profile.is_some() {
        if Profile::delete(id, &*conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus {
                status_code: Status::InternalServerError.code,
                response: Response {
                    message: String::from(MESSAGE_CAN_NOT_DELETE_DATA),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("Could not find profile with id {}", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
