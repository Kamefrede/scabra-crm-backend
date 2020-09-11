#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate crypto;
extern crate rocket_contrib;

use dotenv::*;
use rocket_contrib::database;

mod db;
mod models;
mod proxies;
mod routes;
mod schema;

//TODO: Probably need to move this to the db crate?
#[database("mainDb")]
pub struct CrmDbConn(diesel::PgConnection);

/// #Actual entry endpoint
/// Launching logic is handled here so that
/// main.rs doesn't expose any of our internals.
///
pub fn launch() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .attach(CrmDbConn::fairing())
        .mount("/", routes![routes::user::create_user])
}
