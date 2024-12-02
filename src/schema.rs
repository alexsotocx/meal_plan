// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "MealTypeSql"))]
    pub struct MealTypeSql;
}

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
    use diesel::sql_types::*;
    use super::sql_types::MealTypeSql;

    meal_plan_items (id) {
        id -> Uuid,
        meal_plan_id -> Uuid,
        recipe_id -> Uuid,
        date -> Date,
        servings -> Int2,
        meal_type -> MealTypeSql,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    meal_plans (id) {
        id -> Uuid,
        week -> Int2,
        year -> Int2,
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
    meal_plan_items,
    meal_plans,
    recipes,
);
