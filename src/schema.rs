table! {
    address (id) {
        id -> Int4,
        name -> Varchar,
        line1 -> Varchar,
        line2 -> Nullable<Varchar>,
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
    }
}

table! {
    employee (id) {
        id -> Int4,
        client_id -> Int4,
    }
}

table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    profile (person_id) {
        person_id -> Int4,
        displayname -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        role -> Nullable<Varchar>,
        address_id -> Nullable<Int4>,
    }
}

table! {
    user (id) {
        id -> Int4,
        person_id -> Nullable<Int4>,
        username -> Varchar,
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

joinable!(employee -> client (client_id));
joinable!(employee -> person (id));
joinable!(profile -> address (address_id));
joinable!(profile -> person (person_id));
joinable!(user -> person (person_id));
joinable!(user_auth_token -> user (user_id));

allow_tables_to_appear_in_same_query!(
    address,
    client,
    employee,
    person,
    profile,
    user,
    user_auth_token,
);
