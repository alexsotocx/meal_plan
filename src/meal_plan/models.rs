use diesel::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub password: String,
    pub family_id: u32,
}

#[derive(Debug)]
pub struct Family {
    pub id: u32,
}

#[derive(Debug)]
pub struct MealPlan {
    pub id: u32,
    pub week: u8,
}

#[derive(Debug)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
}

#[derive(Debug)]
pub struct PlanItem {
    pub id: u32,
    pub date: String,
    pub meal_type: MealType,
    pub servings: u8,
    pub recipe_id: u32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    id: uuid::Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    servings: i16,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::ingridients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingridient {
    id: uuid::Uuid,
    name: String,
    unit: String,
    quantity: bigdecimal::BigDecimal,
    recipe_id: uuid::Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
