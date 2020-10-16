use crate::schema::client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Insertable, AsChangeset, FromForm)]
#[table_name = "client"]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub address_id: i32,
    pub client_type: String,
    pub nif: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset)]
#[table_name = "client"]
pub struct ClientEntity {
    pub name: String,
    pub address_id: i32,
    pub client_type: String,
    pub nif: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, EnumString, Display)]
pub enum ClientQueryType {
    #[strum(serialize = "nif")]
    Nif,
    #[strum(serialize = "address_id")]
    AddressId,
    #[strum(serialize = "client_type")]
    ClientType,
    #[strum(serialize = "name")]
    Name,
}
