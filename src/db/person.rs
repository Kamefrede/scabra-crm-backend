use crate::models::client::Client;
use crate::models::person::{Person, PersonEntity};
use crate::schema::person::dsl::{id, name, person};
use diesel::prelude::*;

impl Person {
    pub fn find_all(conn: &PgConnection) -> Vec<Self> {
        person.order(id.asc()).load::<Self>(conn).unwrap()
    }

    pub fn find_by_id(person_id: i32, conn: &PgConnection) -> Option<Self> {
        person.find(person_id).get_result::<Self>(conn).ok()
    }

    pub fn find_by_name(person_name: &str, conn: &PgConnection) -> Option<Vec<Self>> {
        person
            .filter(name.eq(person_name))
            .load::<Self>(conn)
            .map_or(None, |people_vec| {
                if people_vec.is_empty() {
                    None
                } else {
                    Some(people_vec)
                }
            })
    }

    pub fn find_all_employees_by_company(company_id: i32, conn: &PgConnection) -> Vec<Self> {
        let option_client = Client::find_by_id(company_id, conn);
        if let Some(clnt) = option_client {
            Self::belonging_to(&clnt).load::<Self>(conn).unwrap()
        } else {
            vec![]
        }
    }

    pub fn insert(new_person: &PersonEntity, conn: &PgConnection) -> bool {
        diesel::insert_into(person)
            .values(new_person)
            .execute(conn)
            .is_ok()
    }

    pub fn update(person_id: i32, updated_person: &PersonEntity, conn: &PgConnection) -> bool {
        diesel::update(person.find(person_id))
            .set(updated_person)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(person_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(person.find(person_id)).execute(conn).is_ok()
    }
}
