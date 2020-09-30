#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
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
#[macro_use]
extern crate log;
extern crate anyhow;
extern crate pretty_env_logger;
extern crate web_ical;

use diesel::prelude::*;

mod calendar;
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
#[must_use]
pub fn launch() -> rocket::Rocket {
    use crate::calendar::get_or_create_calendar;
    use crate::models::calendar::CalendarState;
    use db::CrmDbConn;
    use dotenv::dotenv;
    use std::sync::{Arc, Mutex};
    pretty_env_logger::init();
    dotenv().ok();
    let rocket = rocket::ignite()
        .manage(CalendarState {
            calendar: Arc::new(Mutex::new(get_or_create_calendar())),
        })
        .attach(CrmDbConn::fairing())
        .mount(
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
                routes::profile::find_by_id,
                routes::profile::query,
                routes::profile::insert,
                routes::profile::update,
                routes::profile::delete,
                routes::profile::find_all,
                routes::client::find_all,
                routes::client::find_by_id,
                routes::client::query,
                routes::client::insert,
                routes::client::update,
                routes::client::delete,
                routes::employee::find_all,
                routes::employee::find_by_id,
                routes::employee::find_all_employees_by_company,
                routes::employee::insert,
                routes::employee::update,
                routes::employee::delete,
                routes::calendar::find_all_events,
                routes::calendar::update,
                routes::calendar::delete,
                routes::calendar::insert,
                routes::calendar::get_last_uid,
                routes::calendar::get_last_event,
                routes::calendar::get_event_by_id,
                routes::calendar::query,
            ],
        );
    cleanup_old_tokens(&CrmDbConn::get_one(&rocket).unwrap());

    rocket
}

fn cleanup_old_tokens(conn: &PgConnection) {
    use crate::models::user_auth_token::UserAuthToken;
    use crate::schema::user_auth_token::dsl::{expires_at, user_auth_token};
    use chrono::Utc;
    let old_tokens = user_auth_token.filter(expires_at.le(Utc::now().naive_utc()));
    for token in old_tokens.load::<UserAuthToken>(conn).unwrap() {
        info!(
            "{}",
            format!(
                "Deleting token for user {} that expired at {:?}",
                token.user_id, token.expires_at
            )
        )
    }
    diesel::delete(old_tokens).execute(conn).unwrap();
}
