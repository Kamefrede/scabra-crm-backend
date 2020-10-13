use crate::models::client::Client;
use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::schema::task;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Identifiable, Queryable, Insertable, AsChangeset, FromForm, Associations,
)]
#[belongs_to(Client)]
#[table_name = "task"]
pub struct Task {
    pub id: i32,
    pub client_id: i32,
    pub start_time: Option<NaiveDateForm>,
    pub end_time: Option<NaiveDateForm>,
    pub status: Option<String>,
    pub description: String,
    pub user_id: Option<i32>,
    pub sync_with_calendar: Option<bool>,
    pub created: NaiveDateForm,
    pub summary: Option<String>,
    pub location: Option<String>,
    pub calendar_uid: Option<i32>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, FromForm, Clone)]
#[table_name = "task"]
pub struct TaskEntity {
    pub client_id: i32,
    pub start_time: Option<NaiveDateForm>,
    pub end_time: Option<NaiveDateForm>,
    pub status: Option<String>,
    pub description: String,
    pub user_id: Option<i32>,
    pub sync_with_calendar: Option<bool>,
    pub created: NaiveDateForm,
    pub summary: Option<String>,
    pub location: Option<String>,
    pub calendar_uid: Option<i32>,
}

#[derive(Serialize, Deserialize, EnumString, Display)]
pub enum TaskQueryType {
    #[strum(serialize = "description")]
    Description,
    #[strum(serialize = "status")]
    Status,
}
