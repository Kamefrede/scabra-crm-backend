use crate::schema::address;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Insertable, Queryable, Identifiable)]
#[table_name="address"]
pub struct Address {
    pub id: i32,
    pub name: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub postal_code: String,
    pub country: String,
    pub address_type: String,
}

#[derive(Deserialize, Serialize, Insertable, AsChangeset, FromForm)]
#[table_name="address"]
pub struct AddressEntity {
    pub name: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub postal_code: String,
    pub country: String,
    pub address_type: String,
}

#[derive(Deserialize, Serialize, FromForm)]
pub struct AddressQuery {
    name: Option<String>,
    line: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    address_type: Option<String>
}
