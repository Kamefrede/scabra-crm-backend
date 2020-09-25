use crate::db::CrmDbConn;
use crate::models::profile::Profile;
use crate::models::response::ResponseWithStatus;
use crate::models::Query;

pub fn find_all(conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Profile::find_all(&**conn))
}

pub fn find_by_id(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_profile = Profile::find_by_id(id, &**conn);
    if let Some(profile) = option_profile {
        ResponseWithStatus::ok_with_data(profile)
    } else {
        ResponseWithStatus::eror_not_found(format!("Could not find profile with id {}", id))
    }
}

pub fn insert(profile: &Profile, conn: &CrmDbConn) -> ResponseWithStatus {
    if Profile::insert(profile, &**conn) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_insert()
    }
}

pub fn query(query: Query, conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Profile::query(query, &**conn))
}

pub fn update(id: i32, new_profile: &Profile, conn: &CrmDbConn) -> ResponseWithStatus {
    if Profile::find_by_id(id, &**conn).is_some() {
        if Profile::update(id, new_profile, &**conn) {
            ResponseWithStatus::error_update()
        } else {
            ResponseWithStatus::ok_empty()
        }
    } else {
        ResponseWithStatus::eror_not_found(format!("Could not find profile with id {}", id))
    }
}

pub fn delete(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if Profile::find_by_id(id, &**conn).is_some() {
        if Profile::delete(id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::eror_not_found(format!("Could not find profile with id {}", id))
    }
}
