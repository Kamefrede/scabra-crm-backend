use crate::schema::person;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable, Identifiable, Queryable)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "person"]
pub struct PersonEntity {
    pub name: String,
}

impl Person {
    pub fn find_all(conn: &PgConnection) -> Vec<Person> {
        use crate::schema::person::dsl::*;
        person.order(id.asc()).load::<Person>(conn).unwrap()
    }

    pub fn find_by_id(person_id: i32, conn: &PgConnection) -> Option<Person> {
        use crate::schema::person::dsl::*;
        let result = person.find(person_id).get_result::<Person>(conn);
        if let Ok(p) = result {
            Some(p)
        } else {
            None
        }
    }

    pub fn find_by_name(person_name: &str, conn: &PgConnection) -> Option<Vec<Person>> {
        use crate::schema::person::dsl::*;
        let people = person.filter(name.eq(person_name)).load::<Person>(conn);
        if let Ok(people_vec) = people {
            if people_vec.is_empty() {
                None
            } else {
                Some(people_vec)
            }
        } else {
            None
        }
    }

    pub fn insert(new_person: PersonEntity, conn: &PgConnection) -> bool {
        use crate::schema::person::dsl::*;
        diesel::insert_into(person)
            .values(&new_person)
            .execute(conn)
            .is_ok()
    }

    pub fn update(person_id: i32, updated_person: PersonEntity, conn: &PgConnection) -> bool {
        use crate::schema::person::dsl::*;
        diesel::update(person.find(person_id))
            .set(&updated_person)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(person_id: i32, conn: &PgConnection) -> bool {
        use crate::schema::person::dsl::*;
        diesel::delete(person.find(person_id)).execute(conn).is_ok()
    }
}
