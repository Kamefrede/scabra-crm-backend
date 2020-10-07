use crate::db::CrmDbConn;
use crate::models::person::{Person, PersonEntity};
use crate::models::response::ResponseWithStatus;

pub fn find_all(conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Person::find_all(&**conn))
}

pub fn find_by_id(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_id(id, &**conn);
    if let Some(person) = option_person {
        ResponseWithStatus::ok_with_data(person)
    } else {
        ResponseWithStatus::error_not_found(format!("No people with id {} was found", id))
    }
}

pub fn find_by_name(name: &str, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_person = Person::find_by_name(name, &**conn);
    if let Some(person) = option_person {
        ResponseWithStatus::ok_with_data(person)
    } else {
        ResponseWithStatus::error_not_found(format!("No people with name {} was found", &name))
    }
}

pub fn insert(person: &PersonEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if Person::insert(person, &**conn) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_insert()
    }
}

pub fn update(id: i32, updated_person: &PersonEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if Person::find_by_id(id, &**conn).is_some() {
        if Person::update(id, updated_person, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_update()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("No person with id {} was found", id))
    }
}

pub fn delete(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if Person::find_by_id(id, &**conn).is_some() {
        if Person::delete(id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("No person with id {} was found", id))
    }
}
