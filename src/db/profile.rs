use crate::models::profile::{Profile, ProfileQueryType};
use crate::models::Query;
use crate::schema::profile::dsl::{displayname, person_id, phone_number, profile};
use diesel::prelude::*;
use std::string::ToString;

impl Profile {
    pub fn find_all(conn: &PgConnection) -> Vec<Self> {
        profile.order(person_id.asc()).load::<Self>(conn).unwrap()
    }

    pub fn find_by_id(id: i32, conn: &PgConnection) -> Option<Self> {
        profile.find(id).get_result::<Self>(conn).ok()
    }

    pub fn insert(new_profile: &Self, conn: &PgConnection) -> bool {
        diesel::insert_into(profile)
            .values(new_profile)
            .execute(conn)
            .is_ok()
    }

    //TODO: Implement fuzzy query searching
    pub fn query(query: Query, conn: &PgConnection) -> Vec<Self> {
        match query.query_type {
            x if x == ProfileQueryType::DisplayName.to_string() => profile
                .filter(displayname.eq(&query.query_text))
                .order(person_id.asc())
                .load::<Self>(conn)
                .unwrap(),
            x if x == ProfileQueryType::AddressId.to_string()
                && query.query_text.parse::<i32>().is_ok() =>
            {
                let profile_option =
                    Self::find_by_id(query.query_text.parse::<i32>().unwrap(), conn);
                profile_option.map_or(vec![], |prof| vec![prof])
            }
            x if x == ProfileQueryType::PhoneNumber.to_string() => profile
                .filter(phone_number.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            x if x == ProfileQueryType::Role.to_string() => profile
                .filter(phone_number.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            _ => return vec![],
        }
    }

    pub fn update(id: i32, updated_profile: &Self, conn: &PgConnection) -> bool {
        diesel::update(profile.find(id))
            .set(updated_profile)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(profile.find(id)).execute(conn).is_ok()
    }
}
