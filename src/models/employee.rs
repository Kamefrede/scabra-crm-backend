use crate::schema::employee;
use crate::models::client::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromForm, Queryable, Identifiable, AsChangeset, Insertable, Associations)]
#[table_name = "employee"]
#[belongs_to(Client)]
pub struct Employee {
    id: i32,
    client_id: i32,
}