#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate crypto;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod constants;
mod db;
mod models;
mod proxies;
mod routes;
mod schema;
mod services;

/// #Actual entry endpoint
/// Launching logic is handled here so that
/// main.rs doesn't expose any of our internals.
///
pub fn launch() -> rocket::Rocket {
    use db::CrmDbConn;
    use dotenv::dotenv;
    dotenv().ok();
    rocket::ignite().attach(CrmDbConn::fairing()).mount(
        "/",
        routes![
            routes::user::login,
            routes::user::signup,
            routes::person::find_by_id,
            routes::person::find_by_name,
            routes::person::insert,
            routes::person::update,
            routes::person::delete,
            routes::person::find_all,
            routes::address::find_by_id,
            routes::address::query,
            routes::address::insert,
            routes::address::update,
            routes::address::delete,
            routes::address::find_all,
        ],
    )
}
