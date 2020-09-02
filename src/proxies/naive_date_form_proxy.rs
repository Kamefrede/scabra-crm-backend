use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ops::Deref;
use std::io::Write;
use diesel::pg::data_types::{PgTimestamp};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Timestamp, Timestamptz};
use serde::{Serialize, Deserialize};

/*
*/
#[derive(AsExpression, FromSqlRow, Debug, Serialize, Deserialize)]
#[sql_type = "diesel::sql_types::Timestamptz"]
pub struct NaiveDateForm(NaiveDateTime);

impl Deref for NaiveDateForm {
    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }

    type Target = NaiveDateTime;
}

impl NaiveDateForm {
    pub fn new(naive_date_time: NaiveDateTime) -> NaiveDateForm {
        NaiveDateForm(naive_date_time)
    }
}

/*
    The following section of code was taken from
    https://github.com/7omb/chrono/blob/rocket/src/naive/time.rs
    https://github.com/7omb/chrono/blob/rocket/src/naive/date.rs
    https://github.com/7omb/chrono/blob/rocket/src/naive/datetime.rs
    Which is dual-licensed under the MIT and Apache License(https://github.com/chronotope/chrono/blob/main/LICENSE.txt)
*/


impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if decoded.len() < "0000-00-00T00:00".len() {
            return Err(form_value);
        }
        let date = naive_date_from_form_value(RawStr::from_str(&decoded[.."0000-00-00".len()]))
            .map_err(|_| form_value)?;
        let time = naive_time_from_form_value(RawStr::from_str(&decoded["0000-00-00T".len()..]))
            .map_err(|_| form_value)?;
        Ok(NaiveDateForm(NaiveDateTime::new(date, time)))
    }
}


fn naive_date_from_form_value<'v>(form_value: &'v RawStr) -> Result<NaiveDate, &'v RawStr> {
    let decoded = form_value.url_decode().map_err(|_| form_value)?;
    if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
        return Ok(date);
    }
    Err(form_value)
}

fn naive_time_from_form_value<'v>(form_value: &'v RawStr) -> Result<NaiveTime, &'v RawStr> {
    let decoded = form_value.url_decode().map_err(|_| form_value)?;
    if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M:%S%.3f") {
        use chrono::Timelike;
        if time.nanosecond() >= 1_000_000_000 {
            return Err(form_value);
        }
        return Ok(time);
    }
    if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M") {
        return Ok(time);
    }
    Err(form_value)
}

/*
    The following section of code was copied(with some minor alteration) from
    https://github.com/diesel-rs/diesel/blob/master/diesel/src/pg/types/date_and_time/chrono.rs
    Which is licensed under the Apache License(https://github.com/diesel-rs/diesel/blob/master/LICENSE-APACHE)
*/
fn pg_epoch() -> NaiveDateTime {
    NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0)
}

impl FromSql<Timestamp, Pg> for NaiveDateForm {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let PgTimestamp(offset) = FromSql::<Timestamp, Pg>::from_sql(bytes)?;
        match pg_epoch().checked_add_signed(Duration::microseconds(offset)) {
            Some(v) => Ok(NaiveDateForm(v)),
            None => {
                let message = "Tried to deserialize a timestamp that is too large for Chrono";
                Err(message.into())
            }
        }
    }
}

impl ToSql<Timestamp, Pg> for NaiveDateForm {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let time = match (self.deref().signed_duration_since(pg_epoch())).num_microseconds() {
            Some(time) => time,
            None => {
                let error_message =
                    format!("{:?} as microseconds is too large to fit in an i64", self);
                return Err(error_message.into());
            }
        };
        ToSql::<Timestamp, Pg>::to_sql(&PgTimestamp(time), out)
    }
}

impl FromSql<Timestamptz, Pg> for NaiveDateForm {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        FromSql::<Timestamp, Pg>::from_sql(bytes)
    }
}

impl ToSql<Timestamptz, Pg> for NaiveDateForm {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        ToSql::<Timestamp, Pg>::to_sql(self, out)
    }
}