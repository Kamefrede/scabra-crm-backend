use diesel::pg::PgConnection;
use rocket_contrib::database;

pub mod user;
pub mod user_auth_token;

#[database("mainDb")]
pub struct CrmDbConn(PgConnection);
