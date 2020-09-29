use crate::schema::client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Insertable, AsChangeset, FromForm)]
#[table_name = "client"]
pub struct Client {
    id: i32,
    name: String,
    address_id: i32,
    client_type: String,
    nif: String,
}

#[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset)]
#[table_name = "client"]
pub struct ClientEntity {
    name: String,
    address_id: i32,
    client_type: String,
    nif: String,
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
