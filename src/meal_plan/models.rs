use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

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

#[derive(Debug, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub servings: i16,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::ingridients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingridient {
    pub id: uuid::Uuid,
    pub name: String,
    pub unit: String,
    pub quantity: bigdecimal::BigDecimal,
    pub recipe_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
