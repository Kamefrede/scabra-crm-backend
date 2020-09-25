use crate::schema::person;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Identifiable, Queryable)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, FromForm)]
#[table_name = "person"]
pub struct PersonEntity {
    pub name: String,
}
