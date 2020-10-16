use crate::schema::address;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Queryable, Identifiable)]
#[table_name = "address"]
pub struct Address {
    pub id: i32,
    pub name: String,
    pub address_line: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
    pub address_type: String,
}

#[derive(Deserialize, Serialize, Insertable, AsChangeset, FromForm)]
#[table_name = "address"]
pub struct AddressEntity {
    pub name: String,
    pub address_line: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
    pub address_type: String,
}
