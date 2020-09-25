use crate::schema::address;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Deserialize, Serialize, Insertable, Queryable, Identifiable)]
#[table_name = "address"]
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
#[table_name = "address"]
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
    pub query_type: String,
    pub query_text: String,
}

#[derive(Deserialize, Serialize, EnumString, Display)]
pub enum AddressQueryType {
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "line")]
    Line,
    #[strum(serialize = "city")]
    City,
    #[strum(serialize = "postal_code")]
    PostalCode,
    #[strum(serialize = "country")]
    Country,
    #[strum(serialize = "address_type")]
    AddressType,
}
