use crate::models::client::{Client, ClientEntity};
use crate::schema::client::dsl::{client, id};
use diesel::prelude::*;

impl Client {
    pub fn find_all(conn: &PgConnection) -> Vec<Self> {
        client.order(id.asc()).load::<Self>(conn).unwrap()
    }

    pub fn find_by_id(client_id: i32, conn: &PgConnection) -> Option<Self> {
        client.find(client_id).get_result::<Self>(conn).ok()
    }

    pub fn insert(new_client: &ClientEntity, conn: &PgConnection) -> bool {
        diesel::insert_into(client)
            .values(new_client)
            .execute(conn)
            .is_ok()
    }

    pub fn update(updated_client: &ClientEntity, client_id: i32, conn: &PgConnection) -> bool {
        diesel::update(client.find(client_id))
            .set(updated_client)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(client_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(client.find(client_id)).execute(conn).is_ok()
    }
}
