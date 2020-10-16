use crate::models::address::{Address, AddressEntity};
use crate::schema::address::dsl::{address, id};
use diesel::prelude::*;

impl Address {
    pub fn find_all(conn: &PgConnection) -> Vec<Self> {
        address.order(id.asc()).load::<Self>(conn).unwrap()
    }

    pub fn find_by_id(address_id: i32, conn: &PgConnection) -> Option<Self> {
        address.find(address_id).get_result::<Self>(conn).ok()
    }

    pub fn insert(new_address: &AddressEntity, conn: &PgConnection) -> bool {
        diesel::insert_into(address)
            .values(new_address)
            .execute(conn)
            .is_ok()
    }

    pub fn update(address_id: i32, updated_address: &AddressEntity, conn: &PgConnection) -> bool {
        diesel::update(address.find(address_id))
            .set(updated_address)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(address_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(address.find(address_id))
            .execute(conn)
            .is_ok()
    }
}
