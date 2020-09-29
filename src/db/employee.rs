use crate::models::employee::Employee;
use crate::schema::employee::dsl::{employee, id};
use diesel::prelude::*;
use crate::models::client::Client;

impl Employee {
    pub fn find_all(conn: &PgConnection) -> Vec<Self> {
        employee.order(id.asc())
            .load::<Self>(conn)
            .unwrap()
    }

    pub fn find_by_id(employee_id: i32, conn: &PgConnection) -> Option<Self> {
        employee.find(employee_id)
            .get_result::<Self>(conn)
            .ok()
    }

    pub fn find_all_employees_by_company(company_id: i32, conn: &PgConnection) -> Vec<Self> {
        let option_client = Client::find_by_id(company_id, conn);
        if let Some(clnt) = option_client {
            Self::belonging_to(&clnt)
                .load::<Self>(conn)
                .unwrap()
        } else {
            vec![]
        }
    }

    pub fn insert(new_employee: &Self, conn: &PgConnection) -> bool {
        diesel::insert_into(employee)
            .values(new_employee)
            .execute(conn)
            .is_ok()
    }

    pub fn update(updated_employee: &Self, employee_id: i32, conn: &PgConnection) -> bool {
        diesel::update(employee.find(employee_id))
            .set(updated_employee)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(employee_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(employee.find(employee_id))
            .execute(conn)
            .is_ok()
    }
}
