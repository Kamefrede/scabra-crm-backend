use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::deserialize::{self, FromSql};
use diesel::pg::data_types::PgTimestamp;
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Timestamp, Timestamptz};
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::ops::Deref;

/*
    The following section of code was taken from
    https://github.com/diesel-rs/diesel/blob/master/diesel/src/type_impls/date_and_time.rs
    Which is licensed under the Apache License(https://github.com/diesel-rs/diesel/blob/master/LICENSE-APACHE)
*/
#[derive(
    AsExpression,
    FromSqlRow,
    Debug,
    Serialize,
    Deserialize,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[sql_type = "diesel::sql_types::Timestamptz"]
pub struct NaiveDateForm(NaiveDateTime);

impl Deref for NaiveDateForm {
    type Target = NaiveDateTime;

    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }
}

impl NaiveDateForm {
    pub(crate) const fn new(naive_date_time: NaiveDateTime) -> Self {
        Self(naive_date_time)
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

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        let decode_result = form_value.url_decode();
        if let Ok(decoded) = decode_result {
            if decoded.len() < "0000-00-00T00:00".len() {
                return Err(form_value);
            }
            let date_result =
                naive_date_from_form_value(RawStr::from_str(&decoded[.."0000-00-00".len()]));
            let time_result =
                naive_time_from_form_value(RawStr::from_str(&decoded["0000-00-00T".len()..]));
            if let (Ok(date), Ok(time)) = (date_result, time_result) {
                return Ok(Self(NaiveDateTime::new(date, time)));
            }
        }
        Err(form_value)
    }
}

fn naive_date_from_form_value(form_value: &'_ RawStr) -> Result<NaiveDate, &'_ RawStr> {
    let decode_result = form_value.url_decode();
    if let Ok(decoded) = decode_result {
        if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
            return Ok(date);
        }
    }
    Err(form_value)
}

fn naive_time_from_form_value(form_value: &'_ RawStr) -> Result<NaiveTime, &'_ RawStr> {
    let decode_result = form_value.url_decode();
    if let Ok(decoded) = decode_result {
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
        pg_epoch()
            .checked_add_signed(Duration::microseconds(offset))
            .map_or_else(
                || {
                    let message = "Tried to deserialize a timestamp that is too large for Chrono";
                    Err(message.into())
                },
                |v| Ok(Self(v)),
            )
    }
}

impl ToSql<Timestamp, Pg> for NaiveDateForm {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        if let Some(time) = (self.deref().signed_duration_since(pg_epoch())).num_microseconds() {
            ToSql::<Timestamp, Pg>::to_sql(&PgTimestamp(time), out)
        } else {
            let error_message = format!("{:?} as microseconds is too large to fit in an i64", self);
            Err(error_message.into())
        }
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
