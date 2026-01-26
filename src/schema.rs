// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Uuid,
        name -> Text,
        cpf -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(persons, users,);
