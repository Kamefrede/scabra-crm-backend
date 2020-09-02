#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod naive_date_form_proxy;