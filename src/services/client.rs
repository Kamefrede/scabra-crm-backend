use crate::db::CrmDbConn;
use crate::models::client::{Client, ClientEntity};
use crate::models::response::ResponseWithStatus;

pub fn find_all(conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Client::find_all(&**conn))
}

pub fn find_by_id(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_client = Client::find_by_id(id, &**conn);
    if let Some(client) = option_client {
        ResponseWithStatus::ok_with_data(client)
    } else {
        ResponseWithStatus::error_not_found(format!("Client with id {} was not found", id))
    }
}

pub fn insert(client: &ClientEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if Client::insert(client, &**conn) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_insert()
    }
}

pub fn update(id: i32, updated_client: &ClientEntity, conn: &CrmDbConn) -> ResponseWithStatus {
    if Client::find_by_id(id, &**conn).is_some() {
        if Client::update(updated_client, id, &**conn) {
            ResponseWithStatus::error_update()
        } else {
            ResponseWithStatus::ok_empty()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("Could not find client with id {}", id))
    }
}

pub fn delete(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if Client::find_by_id(id, &**conn).is_some() {
        if Client::delete(id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::error_not_found(format!("Could not find client with id {}", id))
    }
}
