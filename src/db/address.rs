use crate::models::address::{Address, AddressEntity, AddressQueryType};
use crate::models::Query;
use crate::schema::address::dsl::{
    address, address_type, city, country, id, line1, line2, name, postal_code,
};
use diesel::prelude::*;
use std::string::ToString;

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

    //TODO: Fuzzy searching from DB
    pub fn query(query: Query, conn: &PgConnection) -> Vec<Self> {
        match query.query_type {
            x if x == AddressQueryType::Name.to_string() => address
                .filter(name.eq(&query.query_text))
                .order(id.asc())
                .load::<Self>(conn)
                .unwrap(),
            x if x == AddressQueryType::Line.to_string() => address
                .filter(line1.eq(&query.query_text))
                .or_filter(line2.eq(&query.query_text))
                .order(id.asc())
                .load::<Self>(conn)
                .unwrap(),
            x if x == AddressQueryType::City.to_string() => address
                .filter(city.eq(&query.query_text))
                .order(id.asc())
                .load::<Self>(conn)
                .unwrap(),
            x if x == AddressQueryType::PostalCode.to_string() => address
                .filter(postal_code.eq(&query.query_text))
                .order(id.asc())
                .load::<Self>(conn)
                .unwrap(),
            x if x == AddressQueryType::Country.to_string() => address
                .filter(country.eq(&query.query_text))
                .order(id.asc())
                .load::<Self>(conn)
                .unwrap(),
            x if x == AddressQueryType::AddressType.to_string() => address
                .filter(address_type.eq(&query.query_text))
                .order(id.asc())
                .load::<Self>(conn)
                .unwrap(),
            _ => return vec![],
        }
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
