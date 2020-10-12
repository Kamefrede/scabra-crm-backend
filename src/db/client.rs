use crate::models::client::{Client, ClientEntity, ClientQueryType};
use crate::models::Query;
use crate::schema::client::dsl::{address_id, client, client_type, id, name, nif};
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

    //TODO: Implement fuzzy searching
    pub fn query(query: &Query, conn: &PgConnection) -> Vec<Self> {
        match (*query.query_type).to_string() {
            x if x == ClientQueryType::AddressId.to_string()
                && query.query_text.parse::<i32>().is_ok() =>
            {
                client
                    .filter(address_id.eq(query.query_text.parse::<i32>().unwrap()))
                    .load::<Self>(conn)
                    .unwrap()
            }
            x if x == ClientQueryType::Name.to_string() => client
                .filter(name.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            x if x == ClientQueryType::ClientType.to_string() => client
                .filter(client_type.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            x if x == ClientQueryType::Nif.to_string() => client
                .filter(nif.eq(&query.query_text))
                .load::<Self>(conn)
                .unwrap(),
            _ => vec![],
        }
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
