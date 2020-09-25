use crate::models::profile::{Profile, ProfileQueryType};
use crate::models::Query;
use crate::schema::profile::dsl::*;
use diesel::prelude::*;
use std::string::ToString;


impl Profile {

    pub fn find_all(conn: &PgConnection) -> Vec<Profile> {
        profile.order(person_id.asc()).load::<Profile>(conn).unwrap()
    }

    pub fn find_by_id(id: i32, conn: &PgConnection) -> Option<Profile> {
        let result = profile.find(id).get_result::<Profile>(conn);

        if let Ok(prof) = result {
            Some(prof)
        } else {
            None
        }
    }

    pub fn insert(new_profile: Profile, conn: &PgConnection) -> bool {
        diesel::insert_into(profile)
            .values(&new_profile)
            .execute(conn)
            .is_ok()
    }

    //TODO: Implement fuzzy query searching
    pub fn query(query: Query, conn: &PgConnection) -> Vec<Profile> {
        match query.query_type {
            x if x == ProfileQueryType::DisplayName.to_string() =>
            profile.filter(displayname.eq(&query.query_text))
                .order(person_id.asc())
                .load::<Profile>(conn)
                .unwrap(),
            x if x == ProfileQueryType::AddressId.to_string() && x.parse::<i32>().is_ok()=> {
                let profile_option = Self::find_by_id(x.parse::<i32>().unwrap(), conn);
                if let Some(prof) = profile_option {
                    vec![prof]
                } else {
                    vec![]
                }
            },
            x if x == ProfileQueryType::PhoneNumber.to_string() =>
                profile.filter(phone_number.eq(&query.query_text))
                    .load::<Profile>(conn)
                    .unwrap(),
            x if x == ProfileQueryType::Role.to_string() =>
                profile.filter(phone_number.eq(&query.query_text))
                    .load::<Profile>(conn)
                    .unwrap(),
            _ => vec![]
        }
    }

    pub fn update(id: i32, updated_profile: Profile, conn: &PgConnection) -> bool {
        diesel::update(profile.find(id))
            .set(&updated_profile)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(profile.find(id))
            .execute(conn)
            .is_ok()
    }
}
