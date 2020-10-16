use std::sync::{Arc, Mutex};
use web_ical::Calendar;

pub struct CalendarState {
    pub calendar: Arc<Mutex<Calendar>>,
}
