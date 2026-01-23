// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Uuid,
        name -> Text,
        cpf -> Text,
    }
}
