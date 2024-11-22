// @generated automatically by Diesel CLI.

diesel::table! {
    ingridients (id) {
        id -> Uuid,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        unit -> Varchar,
        quantity -> Numeric,
        recipe_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    recipes (id) {
        id -> Uuid,
        #[max_length = 256]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        servings -> Int2,
    }
}

diesel::joinable!(ingridients -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    ingridients,
    recipes,
);
