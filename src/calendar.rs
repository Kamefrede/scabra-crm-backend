use crate::models::calendar::EventQueryType;
use crate::models::Query;
use crate::proxies::event_proxy::EventJson;
use std::env;
use web_ical::{Calendar, Events};

const CALENDAR_VERSION: &str = "2.0";
const CALENDAR_PRODID: &str = "-//My Business Inc//My Calendar 70.9054//EN";
const CALENDAR_SCALE: &str = "GREGORIAN";
const CALENDAR_METHOD: &str = "PUBLISH";

/*
 * Gets the calendar as defined in the dotenv CALENDAR_FILE
 * Per user calendar in the future, maybe?
 */
pub fn get_or_create_calendar() -> Calendar {
    let calendar_name =
        env::var("CALENDAR_FILE").expect("CALENDAR_FILE should be defined in the .env file");
    dbg!(env::current_dir().unwrap());
    if let Ok(calendar_str) = std::fs::read_to_string(&calendar_name) {
        if let Ok(calendar) = Calendar::new_from_data(&calendar_str) {
            info!("Existing calendar: {} was found", &calendar_name);
            return calendar;
        }
    }
    info!("No calendar was found, fallbacking to a new one");
    let calendar = create_calendar_from_env();
    if write_calendar_to_file(&calendar_name, &calendar) {
        error!("Errored while writing to file {}", &calendar_name);
    }
    calendar
}

pub fn add_event(calendar: &mut Calendar, event_json: &EventJson) {
    let event = event_json.to_event();
    calendar.add_event(event);
    write_calendar_from_env(calendar);
}

//Future TODO: See if it's needed to add timestamp stuff
//Future TODO: implement fuzzy searching
pub fn query(query: &Query, calendar: &Calendar) -> Vec<Events> {
    match (*query.query_type).to_string() {
        x if x == EventQueryType::Status.to_string() => calendar
            .events
            .iter()
            .cloned()
            .filter(|event| event.status == query.query_text)
            .collect(),
        x if x == EventQueryType::Description.to_string() => calendar
            .events
            .iter()
            .cloned()
            .filter(|event| event.description == query.query_text)
            .collect(),
        x if x == EventQueryType::Location.to_string() => calendar
            .events
            .iter()
            .cloned()
            .filter(|event| event.location == query.query_text)
            .collect(),
        x if x == EventQueryType::Summary.to_string() => calendar
            .events
            .iter()
            .cloned()
            .filter(|event| event.summary == query.query_text)
            .collect(),
        x if x == EventQueryType::Transp.to_string() => calendar
            .events
            .iter()
            .cloned()
            .filter(|event| event.transp == query.query_text)
            .collect(),
        x if x == EventQueryType::Sequence.to_string() && query.query_text.parse::<u32>().is_ok() => {
            calendar
                .events
                .iter()
                .cloned()
                .filter(|event| event.sequence == query.query_text.parse::<u32>().unwrap())
                .collect()
        }
        _ => vec![],
    }
}

pub fn get_last_event<'a>(calendar: &'a Calendar) -> Option<&'a Events> {
    let events = &calendar.events;
    let event: Option<&'a Events> = events.iter().max_by(|event_x, event_y| {
        event_x
            .uid
            .parse::<i32>()
            .unwrap()
            .cmp(&event_y.uid.parse::<i32>().unwrap())
    });
    event
}

pub fn find_event_by_id<'a>(calendar: &'a Calendar, id: i32) -> Option<&'a Events> {
    let events = &calendar.events;
    let event: Option<&'a Events> = events
        .iter()
        .find(|event| event.uid.parse::<i32>().is_ok() && event.uid.parse::<i32>().unwrap() == id);
    event
}

fn get_index_by_id(calendar: &Calendar, id: i32) -> Option<usize> {
    calendar.events.iter().position(|event| {
        event.uid.parse::<i32>().is_ok() && event.uid.parse::<i32>().unwrap() == id
    })
}

pub fn delete_by_id(id: i32, calendar: &mut Calendar) -> bool {
    let index = get_index_by_id(calendar, id);
    index.map_or(false, |idx| {
        calendar.events.remove(idx);
        write_calendar_from_env(calendar);
        true
    })
}

pub fn replace_event_in_calendar(id: i32, event_json: &EventJson, calendar: &mut Calendar) -> bool {
    let index = get_index_by_id(calendar, id);
    let event = event_json.to_event();
    index.map_or(false, |idx| {
        calendar.events[idx] = event;
        write_calendar_from_env(calendar);
        true
    })
}
/*
 * dotenv().ok() must be called before this and CALENDAR_FILE must be defined
 */
pub fn write_calendar_from_env(calendar: &Calendar) -> bool {
    let calendar_file =
        env::var("CALENDAR_FILE").expect("CALENDAR_FILE should be defined in the .env file");
    write_calendar_to_file(&calendar_file, calendar)
}

pub fn write_calendar_to_file(file_path: &str, calendar: &Calendar) -> bool {
    if std::fs::metadata(file_path).is_ok() {
        let mut file = std::fs::File::open(file_path).unwrap();
        match calendar.export_to(&mut file) {
            Ok(_) => true,
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    } else {
        match calendar.export_ics(file_path) {
            Ok(_) => true,
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    }
}

/*
 * dotenv().ok() must be called before this
 * CALENDAR_NAME and CALENDAR_TIMEZONE must also be defined in the .env file
 * probably going to be removed in the future when i add multiple calendars
 * perhaps
 */
pub fn create_calendar_from_env() -> Calendar {
    let calendar_name =
        env::var("CALENDAR_NAME").expect("CALENDAR_NAME should be defined in the .env file");
    let calendar_timezone = env::var("CALENDAR_TIMEZONE")
        .expect("CALENDAR_TIMEZONE should be defined in the .env file");
    create_calendar(&calendar_name, &calendar_timezone)
}

pub fn create_calendar(calendar_name: &str, calendar_timezone: &str) -> Calendar {
    Calendar::create(
        CALENDAR_PRODID,
        CALENDAR_VERSION,
        CALENDAR_SCALE,
        CALENDAR_METHOD,
        calendar_name,
        calendar_timezone,
    )
}

#[test]
fn test_calendar() {
    use dotenv::dotenv;
    dotenv().ok();
    get_or_create_calendar();
}
