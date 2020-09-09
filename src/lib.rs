#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate rocket_contrib;

use rocket_contrib::database;

mod db;
mod models;
mod proxies;
mod routes;
mod schema;

#[database("mainDb")]
pub struct CrmDbConn(diesel::PgConnection);

pub fn launch() -> rocket::Rocket {
    rocket::ignite()
        .attach(CrmDbConn::fairing())
        .mount("/", routes![routes::user::create_user])
}
