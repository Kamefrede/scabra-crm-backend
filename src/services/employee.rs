use crate::db::CrmDbConn;
use crate::models::employee::Employee;
use crate::models::response::ResponseWithStatus;


pub fn find_all(conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Employee::find_all(&**conn))
}

pub fn find_by_id(id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    let option_employee = Employee::find_by_id(id, &**conn);
    if let Some(employee) = option_employee {
        ResponseWithStatus::ok_with_data(employee)
    } else {
        ResponseWithStatus::eror_not_found(format!("Employee with id {} not found", id))
    }
}

pub fn find_all_employees_by_company(company_id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    ResponseWithStatus::ok_with_data(Employee::find_all_employees_by_company(company_id, &**conn))
}

pub fn insert(new_employee: &Employee, conn: &CrmDbConn) -> ResponseWithStatus {
    if Employee::insert(new_employee, &**conn) {
        ResponseWithStatus::ok_empty()
    } else {
        ResponseWithStatus::error_insert()
    }
}

pub fn update(updated_employee: &Employee, employee_id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if Employee::find_by_id(employee_id, &**conn).is_some() {
        if Employee::update(updated_employee, employee_id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_update()
        }
    } else {
        ResponseWithStatus::eror_not_found(format!("Employee with id {} was not found", employee_id))
    }
}

pub fn delete(employee_id: i32, conn: &CrmDbConn) -> ResponseWithStatus {
    if Employee::find_by_id(employee_id, &**conn).is_some() {
        if Employee::delete( employee_id, &**conn) {
            ResponseWithStatus::ok_empty()
        } else {
            ResponseWithStatus::error_delete()
        }
    } else {
        ResponseWithStatus::eror_not_found(format!("Employee with id {} was not found", employee_id))
    }
}