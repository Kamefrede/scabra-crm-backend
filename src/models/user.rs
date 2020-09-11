use diesel::Queryable;

#[derive(Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub person_id: Option<i32>,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}
