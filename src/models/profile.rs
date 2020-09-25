use crate::schema::profile;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Insertable, Queryable, FromForm, AsChangeset)]
#[table_name = "profile"]
pub struct Profile {
    person_id: i32,
    displayname: Option<String>,
    image: Option<String>,
    phone_number: Option<String>,
    role: Option<String>,
    address_id: Option<i32>,
}


#[derive(Deserialize, Serialize, EnumString, Display)]
pub enum ProfileQueryType {
    #[strum(serialize = "display_name")]
    DisplayName,
    #[strum(serialize = "role")]
    Role,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "address_id")]
    AddressId,
}
