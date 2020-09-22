use crate::constants::message_constants::*;
use crate::db::person::{Person, PersonEntity};
use crate::db::CrmDbConn;
use crate::models::response::{Response, ResponseWithStatus};
use rocket::http::Status;

pub fn find_all(conn: CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(MESSAGE_OK),
            data: serde_json::to_value(Person::find_all(&*conn)).unwrap(),
        },
    }
}

pub fn find_by_id(id: i32, conn: CrmDbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &*conn);
    if let Some(person) = option_person {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(MESSAGE_OK),
                data: serde_json::to_value(person).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("person with id {} not found", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn find_by_name(name: String, conn: CrmDbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_name(&name, &*conn);
    if let Some(person) = option_person {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(MESSAGE_OK),
                data: serde_json::to_value(person).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("No people with name {} were found", &name),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn insert(person: PersonEntity, conn: CrmDbConn) -> ResponseWithStatus {
    if Person::insert(person, &*conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
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

pub fn update(id: i32, updated_person: PersonEntity, conn: CrmDbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &*conn);
    if option_person.is_some() {
        if Person::update(id, updated_person, &*conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(MESSAGE_OK),
                    data: serde_json::to_value("").unwrap(),
                },
            }
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
            status_code: Status::BadRequest.code,
            response: Response {
                message: format!("No person with id {} was found", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn delete(id: i32, conn: CrmDbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &*conn);
    if option_person.is_some() {
        if Person::delete(id, &*conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(MESSAGE_OK),
                    data: serde_json::to_value("").unwrap(),
                },
            }
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
            status_code: Status::BadRequest.code,
            response: Response {
                message: format!("No person with id {} was found", id),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
