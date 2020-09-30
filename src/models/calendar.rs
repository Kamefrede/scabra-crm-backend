use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use web_ical::Calendar;

pub struct CalendarState {
    pub calendar: Arc<Mutex<Calendar>>,
}

#[derive(Serialize, Deserialize, EnumString, Display)]
pub enum EventQueryType {
    #[strum(serialize = "dtstart")]
    Dtstart,
    #[strum(serialize = "dtend")]
    Dtend,
    #[strum(serialize = "dtstamp")]
    Dtstamp,
    #[strum(serialize = "created")]
    Created,
    #[strum(serialize = "description")]
    Description,
    #[strum(serialize = "last_modified")]
    LastModified,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "status")]
    Status,
    #[strum(serialize = "summary")]
    Summary,
    #[strum(serialize = "transp")]
    Transp,
    #[strum(serialize = "sequence")]
    Sequence,
}
