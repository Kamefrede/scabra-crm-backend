use crate::constants::message_constants::*;
use crate::db::CrmDbConn;
use crate::models::address::{Address, AddressEntity};
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::Query;
use rocket::http::Status;

pub fn find_all(conn: CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(MESSAGE_OK),
            data: serde_json::to_value(Address::find_all(&*conn)).unwrap(),
        },
    }
}

pub fn find_by_id(id: i32, conn: CrmDbConn) -> ResponseWithStatus {
    let option_address = Address::find_by_id(id, &*conn);
    if let Some(address) = option_address {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(MESSAGE_OK),
                data: serde_json::to_value(address).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("Could not find address with id {}", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn insert(address: AddressEntity, conn: CrmDbConn) -> ResponseWithStatus {
    if Address::insert(address, &*conn) {
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
    ResponseWithStatus::ok_with_data(Address::query(query, &*conn), String::from(MESSAGE_OK))
}

pub fn update(id: i32, new_address: AddressEntity, conn: CrmDbConn) -> ResponseWithStatus {
    let option_address = Address::find_by_id(id, &*conn);
    if option_address.is_some() {
        if Address::update(id, new_address, &*conn) {
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
                message: format!("Could not find address with id {}", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn delete(id: i32, conn: CrmDbConn) -> ResponseWithStatus {
    let option_address = Address::find_by_id(id, &*conn);
    if option_address.is_some() {
        if Address::delete(id, &*conn) {
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
                message: format!("Could not find address with id {}", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
