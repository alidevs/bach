// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        timestamp -> Timestamp,
    }
}
