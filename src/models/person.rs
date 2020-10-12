use crate::models::client::Client;
use crate::schema::person;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Identifiable, Queryable, Associations)]
#[belongs_to(Client)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub image: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub address_id: Option<i32>,
    pub client_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, FromForm)]
#[table_name = "person"]
pub struct PersonEntity {
    pub name: String,
    pub image: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub address_id: Option<i32>,
    pub client_id: Option<i32>,
}

#[derive(Deserialize, Serialize, EnumString, Display)]
pub enum PersonQueryType {
    #[strum(serialize = "role")]
    Role,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "address_id")]
    AddressId,
}
