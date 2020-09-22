use diesel::pg::PgConnection;
use rocket_contrib::database;

pub mod person;
pub mod user;
pub mod user_auth_token;
pub mod address;

#[database("mainDb")]
pub struct CrmDbConn(PgConnection);
