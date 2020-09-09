use diesel::Queryable;

#[derive(Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}
