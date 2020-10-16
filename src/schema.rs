table! {
    address (id) {
        id -> Int4,
        name -> Varchar,
        address_line -> Text,
        city -> Varchar,
        postal_code -> Varchar,
        country -> Varchar,
        address_type -> Varchar,
    }
}

table! {
    client (id) {
        id -> Int4,
        name -> Varchar,
        address_id -> Int4,
        client_type -> Varchar,
        nif -> Varchar,
        email -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
    }
}

table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
        image -> Nullable<Text>,
        phone_number -> Nullable<Varchar>,
        role -> Nullable<Text>,
        address_id -> Nullable<Int4>,
        client_id -> Nullable<Int4>,
        email -> Nullable<Varchar>,
    }
}

table! {
    task (id) {
        id -> Int4,
        client_id -> Int4,
        start_time -> Nullable<Timestamptz>,
        end_time -> Nullable<Timestamptz>,
        status -> Nullable<Text>,
        description -> Text,
        user_id -> Nullable<Int4>,
        sync_with_calendar -> Nullable<Bool>,
        created -> Timestamptz,
        summary -> Nullable<Text>,
        location -> Nullable<Text>,
        calendar_uid -> Nullable<Int4>,
    }
}

table! {
    user (id) {
        id -> Int4,
        person_id -> Nullable<Int4>,
        email -> Varchar,
        hashed_password -> Varchar,
    }
}

table! {
    user_auth_token (user_id) {
        user_id -> Int4,
        login_session -> Varchar,
        generated_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

joinable!(client -> address (address_id));
joinable!(person -> address (address_id));
joinable!(person -> client (client_id));
joinable!(task -> client (client_id));
joinable!(task -> user (user_id));
joinable!(user -> person (person_id));
joinable!(user_auth_token -> user (user_id));

allow_tables_to_appear_in_same_query!(address, client, person, task, user, user_auth_token,);
