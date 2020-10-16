use crate::db::CrmDbConn;
use crate::models::address::{Address, AddressEntity};
use crate::models::response::ResponseWithStatus;

pub fn find_all(conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Address::find_all(&**conn))
}

pub fn find_by_id(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_address = Address::find_by_id(id, &**conn);
    if let Some(address) = option_address {
        ResponseWithStatus::ok_with_data(address)
    } else {
        ResponseWithStatus::error_not_found(format!("Could not find address with id {}", id))
    }
}

pub fn insert(address: &AddressEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if Address::insert(address, &**conn) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_insert()
    }
}

pub fn update(id: i32, new_address: &AddressEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if Address::find_by_id(id, &**conn).is_some() {
        if Address::update(id, new_address, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_update()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("Could not find address with id {}", id))
    }
}

pub fn delete(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if Address::find_by_id(id, &**conn).is_some() {
        if Address::delete(id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("Could not find address with id {}", id))
    }
}
