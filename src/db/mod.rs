use diesel::pg::PgConnection;
use rocket_contrib::database;

pub mod address;
pub mod person;
pub mod user;
pub mod user_auth_token;
pub mod profile;

#[database("mainDb")]
pub struct CrmDbConn(PgConnection);
