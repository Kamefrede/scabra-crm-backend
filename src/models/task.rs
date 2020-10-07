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
    id: i32,
    client_id: i32,
    start_time: Option<NaiveDateForm>,
    end_time: Option<NaiveDateForm>,
    status: Option<String>,
    description: Option<String>,
    user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, FromForm)]
#[table_name = "task"]
pub struct TaskEntity {
    client_id: i32,
    start_time: Option<NaiveDateForm>,
    end_time: Option<NaiveDateForm>,
    status: Option<String>,
    description: Option<String>,
    user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, EnumString, Display)]
pub enum TaskQueryType {
    #[strum(serialize = "description")]
    Description,
    #[strum(serialize = "status")]
    Status,
}
