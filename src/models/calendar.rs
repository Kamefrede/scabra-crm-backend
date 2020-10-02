use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use web_ical::Calendar;

pub struct CalendarState {
    pub calendar: Arc<Mutex<Calendar>>,
}

#[derive(Serialize, Deserialize, EnumString, Display)]
pub enum EventQueryType {
    #[strum(serialize = "description")]
    Description,
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
