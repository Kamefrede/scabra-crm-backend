use crate::calendar;
use crate::models::calendar::CalendarState;
use crate::models::response::ResponseWithStatus;
use crate::models::Query;
use crate::proxies::event_proxy::EventJson;
use rocket::State;

pub fn find_all_events(calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let calendar = calendar_state.calendar.lock().unwrap();
    ResponseWithStatus::ok_with_data(&*calendar.events)
}

pub fn find_by_id(id: i32, calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let calendar = calendar_state.calendar.lock().unwrap();
    let option_event = calendar::find_event_by_id(&calendar, id);
    if let Some(event) = option_event {
        ResponseWithStatus::ok_with_data(event)
    } else {
        ResponseWithStatus::eror_not_found(format!("Couldn't find event with id {}", id))
    }
}

pub fn update(
    id: i32,
    new_event: &EventJson,
    calendar_state: &State<CalendarState>,
) -> ResponseWithStatus {
    let mut calendar = calendar_state.calendar.lock().unwrap();
    if calendar::replace_event_in_calendar(id, new_event, &mut calendar) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_update()
    }
}

pub fn insert(event: &EventJson, calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let mut calendar = calendar_state.calendar.lock().unwrap();
    calendar::add_event(&mut calendar, event);
    ResponseWithStatus::ok_empty()
}

pub fn get_last_uid(calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let calendar = calendar_state.calendar.lock().unwrap();
    let last_event = calendar::get_last_event(&calendar);
    last_event.map_or(
        ResponseWithStatus::eror_not_found(String::from(
            "No event was found! Maybe the calendar is empty?",
        )),
        |event| ResponseWithStatus::ok_with_data(event.uid.parse::<i32>().unwrap()),
    )
}

pub fn get_latest_event(calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let calendar = calendar_state.calendar.lock().unwrap();
    let last_event = calendar::get_last_event(&calendar);
    last_event.map_or(
        ResponseWithStatus::eror_not_found(String::from(
            "No event was found! Maybe the calendar is empty?",
        )),
        ResponseWithStatus::ok_with_data,
    )
}

pub fn query(query: &Query, calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let calendar = calendar_state.calendar.lock().unwrap();
    ResponseWithStatus::ok_with_data(calendar::query(query, &calendar))
}

pub fn delete(id: i32, calendar_state: &State<CalendarState>) -> ResponseWithStatus {
    let mut calendar = calendar_state.calendar.lock().unwrap();
    if calendar::delete_by_id(id, &mut calendar) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::eror_not_found(format!(
            "Couldn't delete because there is no event with the id {}",
            id
        ))
    }
}
