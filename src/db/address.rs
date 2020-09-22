use diesel::prelude::*;
use crate::models::address::{Address, AddressEntity, AddressQuery};
use crate::schema::address::dsl::*;

impl Address {
    pub fn find_all(conn: &PgConnection) -> Vec<Address> {
       address.order(id.asc()).load::<Address>(conn).unwrap()
    }

    pub fn find_by_id(address_id: i32, conn: &PgConnection) -> Option<Address> {
        let result = address.find(address_id).get_result::<Address>(conn);
        if let Ok(addr) = result {
            Some(addr)
        } else {
            None
        }
    }

    pub fn insert(new_address: AddressEntity, conn: &PgConnection) -> bool {
        diesel::insert_into(address)
            .values(&new_address)
            .execute(conn)
            .is_ok()
    }

    pub fn query(query: AddressQuery, conn: &PgConnection) -> Vec<Address> {
        /*match query {
            AddressQuery::Name(value) => {
                address
                    .filter(name.eq(&value))
                    .order(id.asc())
                    .load::<Address>(conn)
                    .unwrap()
            }
            AddressQuery::Line(value) => {
                address
                    .filter(line1.eq(&value))
                    .or_filter(line2.eq(&value))
                    .order(id.asc())
                    .load::<Address>(conn)
                    .unwrap()
            }
            AddressQuery::City(value) => {
                address
                    .filter(city.eq(&value))
                    .order(id.asc())
                    .load::<Address>(conn)
                    .unwrap()
            }
            AddressQuery::PostalCode(value) => {
                address
                    .filter(postal_code.eq(&value))
                    .order(id.asc())
                    .load::<Address>(conn)
                    .unwrap()
            }
            AddressQuery::Country(value) => {
                address
                    .filter(country.eq(&value))
                    .order(id.asc())
                    .load::<Address>(conn)
                    .unwrap()
            }
            AddressQuery::AddressType(value) => {
                address
                    .filter(address_type.eq(&value))
                    .order(id.asc())
                    .load::<Address>(conn)
                    .unwrap()
            }
        }*/
        unimplemented!()
    }

    pub fn update(address_id: i32, updated_address: AddressEntity, conn: &PgConnection) -> bool {
        diesel::update(address.find(address_id))
            .set(&updated_address)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(address_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(address.find(address_id))
            .execute(conn)
            .is_ok()
    }
}