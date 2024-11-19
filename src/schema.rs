// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Uuid,
        #[max_length = 256]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
