use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use rocket::*;
use super::schema::*;
use super::naive_date_form_proxy::{NaiveDateForm};

#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name="users"]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name="articles"]
pub struct Article {
    pub title: String,
    pub body: String,
    pub published_at: NaiveDateForm,
    pub author_id: i32,
}

#[derive(Queryable)]
pub struct ArticleEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published_at: NaiveDateForm,
    pub author_id: i32,
}