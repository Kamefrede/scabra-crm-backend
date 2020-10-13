use crate::models::calendar::CalendarState;
use crate::models::task::{Task, TaskEntity, TaskQueryType};
use crate::models::Query;
use crate::proxies::event_proxy::EventJson;
use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::schema::task::dsl::{client_id, description, id, status, task, user_id};
use diesel::prelude::*;
use rocket::State;

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

    pub fn insert(
        new_task: &TaskEntity,
        conn: &PgConnection,
        state: &State<CalendarState>,
    ) -> bool {
        if Self::is_task_convertable_to_calendar(new_task) {
            let mut calendar = state.calendar.lock().unwrap();
            let mut new_task: TaskEntity = (*new_task).clone();
            new_task.calendar_uid = crate::calendar::get_last_event(&calendar)
                .map_or(Some(0), |cal| Some(cal.uid.parse::<i32>().unwrap() + 1));
            let insertion_result = diesel::insert_into(task)
                .values(&new_task)
                .execute(conn)
                .is_ok();
            if insertion_result {
                if let Some(event) = Self::calendar_event_from_task(&new_task) {
                    crate::calendar::add_event(&mut calendar, &event);
                }
            }
            insertion_result
        } else {
            diesel::insert_into(task)
                .values(new_task)
                .execute(conn)
                .is_ok()
        }
    }
    //TODO: Refresh values after inserting on frontend

    pub fn update(
        updated_task: &TaskEntity,
        task_id: i32,
        conn: &PgConnection,
        state: &State<CalendarState>,
    ) -> bool {
        if Self::is_task_convertable_to_calendar(updated_task) {
            let mut calendar = state.calendar.lock().unwrap();
            if updated_task.calendar_uid.is_some() {
                if let Some(event) = Self::calendar_event_from_task(updated_task) {
                    crate::calendar::replace_event_in_calendar(
                        updated_task.calendar_uid.unwrap(),
                        &event,
                        &mut calendar,
                    );
                }
            } else {
                let mut new_task = (*updated_task).clone();
                new_task.calendar_uid = crate::calendar::get_last_event(&calendar)
                    .map_or(Some(0), |cal| Some(cal.uid.parse::<i32>().unwrap() + 1));
                if let Some(event) = Self::calendar_event_from_task(&new_task) {
                    crate::calendar::replace_event_in_calendar(
                        new_task.calendar_uid.unwrap(),
                        &event,
                        &mut calendar,
                    );
                }
            }
        }
        diesel::update(task.find(task_id))
            .set(updated_task)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(task_id: i32, conn: &PgConnection, state: &State<CalendarState>) -> bool {
        if let Some(to_be_deleted) = Self::find_by_id(task_id, conn) {
            let mut calendar = state.calendar.lock().unwrap();
            if (to_be_deleted.sync_with_calendar.is_some()
                && to_be_deleted.sync_with_calendar.unwrap())
                && (to_be_deleted.calendar_uid.is_some())
            {
                crate::calendar::delete_by_id(to_be_deleted.calendar_uid.unwrap(), &mut calendar);
            }
        }
        diesel::delete(task.find(task_id)).execute(conn).is_ok()
    }

    pub fn calendar_event_from_task(task_to_convert: &TaskEntity) -> Option<EventJson> {
        if let (Some(dtstart), Some(dtend), Some(uid)) = (
            task_to_convert.start_time,
            task_to_convert.end_time,
            task_to_convert.calendar_uid,
        ) {
            let mut event_proxy = EventJson {
                dtstart,
                dtend,
                dtstamp: dtstart,
                uid: uid.to_string(),
                created: task_to_convert.created,
                description: task_to_convert.description.clone(),
                last_modified: NaiveDateForm::new(chrono::Utc::now().naive_utc()),
                location: "".to_string(),
                sequence: 0,
                status: "".to_string(),
                summary: "".to_string(),
                transp: "".to_string(),
            };
            if task_to_convert.location.is_some() {
                event_proxy.location = task_to_convert.location.clone().unwrap();
            }
            if task_to_convert.status.is_some() {
                event_proxy.status = task_to_convert.status.clone().unwrap();
            }
            if task_to_convert.summary.is_some() {
                event_proxy.summary = task_to_convert.summary.clone().unwrap();
            }
            return Some(event_proxy);
        }
        None
    }

    pub const fn is_task_convertable_to_calendar(tsk: &TaskEntity) -> bool {
        tsk.start_time.is_some()
            && tsk.end_time.is_some()
            && (tsk.sync_with_calendar.is_some() && tsk.sync_with_calendar.unwrap())
    }
}
