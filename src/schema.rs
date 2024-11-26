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
    meal_plan_items (id) {
        id -> Uuid,
        meal_plan_id -> Uuid,
        recipe_id -> Uuid,
        date -> Date,
        servings -> Int2,
        meal_type -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    meal_plans (id) {
        id -> Uuid,
        week -> Int2,
        year -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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
    meal_plan_items,
    meal_plans,
    recipes,
);
