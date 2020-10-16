use crate::models::calendar::CalendarState;
use crate::proxies::event_proxy::EventJson;
use crate::routes::{rocket_status_from_response, CustomJsonResponse, JsonWebToken};
use crate::services::calendar;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/calendar")]
pub fn find_all_events(
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = calendar::find_all_events(&calendar_state);
    rocket_status_from_response(response)
}

#[get("/calendar/<id>")]
pub fn get_event_by_id(
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
    id: i32,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = calendar::find_by_id(id, &calendar_state);
    rocket_status_from_response(response)
}

#[post("/calendar", format = "json", data = "<event>")]
pub fn insert(
    event: Json<EventJson>,
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = calendar::insert(&event.0, &calendar_state);
    rocket_status_from_response(response)
}

#[put("/calendar/<id>", format = "json", data = "<event>")]
pub fn update(
    id: i32,
    event: Json<EventJson>,
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = calendar::update(id, &event.0, &calendar_state);
    rocket_status_from_response(response)
}

#[get("/calendar/last/uid")]
pub fn get_last_uid(
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let reponse = calendar::get_last_uid(&calendar_state);
    rocket_status_from_response(reponse)
}

#[get("/calendar/last/event")]
pub fn get_last_event(
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let response = calendar::get_latest_event(&calendar_state);
    rocket_status_from_response(response)
}

#[delete("/calendar/<id>")]
pub fn delete(
    id: i32,
    token: JsonWebToken,
    calendar_state: State<CalendarState>,
) -> CustomJsonResponse {
    if let Err(e) = token {
        return e;
    }
    let reponse = calendar::delete(id, &calendar_state);
    rocket_status_from_response(reponse)
}
