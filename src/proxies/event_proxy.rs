use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use web_ical::Events;

#[derive(Serialize, Deserialize, FromForm)]
pub struct EventJson {
    pub dtstart: NaiveDateForm,
    pub dtend: NaiveDateForm,
    pub dtstamp: NaiveDateForm,
    pub uid: String,
    pub created: NaiveDateForm,
    pub description: String,
    pub last_modified: NaiveDateForm,
    pub location: String,
    pub sequence: u32,
    pub status: String,
    pub summary: String,
    pub transp: String,
}

impl EventJson {
    pub fn to_event(&self) -> Events {
        Events {
            dtstart: DateTime::from_utc(*self.dtstart, Utc),
            dtend: DateTime::from_utc(*self.dtend, Utc),
            dtstamp: DateTime::from_utc(*self.dtstart, Utc),
            uid: self.uid.clone(),
            created: DateTime::from_utc(*self.created, Utc),
            description: self.description.clone(),
            last_modified: DateTime::from_utc(*self.last_modified, Utc),
            location: self.location.clone(),
            sequence: self.sequence,
            status: self.status.clone(),
            summary: self.summary.clone(),
            transp: self.transp.clone(),
        }
    }
}
