use crate::models::task::{Task, TaskEntity, TaskQueryType};
use crate::models::Query;
use crate::schema::task::dsl::{client_id, description, id, status, task, user_id};
use diesel::prelude::*;

impl Task {
    pub fn find_all(conn: &PgConnection) -> Vec<Self> {
        task.order(id.asc()).load::<Self>(conn).unwrap()
    }

    pub fn find_by_id(task_id: i32, conn: &PgConnection) -> Option<Self> {
        task.find(task_id).get_result::<Self>(conn).ok()
    }

    pub fn find_all_user_tasks(id_user: i32, conn: &PgConnection) -> Vec<Self> {
        task.filter(user_id.eq(id_user)).load::<Self>(conn).unwrap()
    }

    pub fn find_all_client_tasks(id_client: i32, conn: &PgConnection) -> Vec<Self> {
        task.filter(client_id.eq(id_client))
            .load::<Self>(conn)
            .unwrap()
    }

    pub fn query(query: &Query, conn: &PgConnection) -> Vec<Self> {
        match (*query.query_type).to_string() {
            x if x == TaskQueryType::Description.to_string() => task
                .filter(description.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            x if x == TaskQueryType::Status.to_string() => task
                .filter(status.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            _ => vec![],
        }
    }

    pub fn insert(new_task: &TaskEntity, conn: &PgConnection) -> bool {
        diesel::insert_into(task)
            .values(new_task)
            .execute(conn)
            .is_ok()
    }

    pub fn update(updated_task: &TaskEntity, task_id: i32, conn: &PgConnection) -> bool {
        diesel::update(task.find(task_id))
            .set(updated_task)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(task_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(task.find(task_id)).execute(conn).is_ok()
    }
}
