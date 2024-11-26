use chrono::{DateTime, Utc};
use diesel::{prelude::*, sql_types::Date};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub password: String,
    pub family_id: Uuid,
}

#[derive(Debug)]
pub struct Family {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct MealPlan {
    pub id: Uuid,
    pub week: i16,
    pub year: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
}

#[derive(Debug)]
pub struct MealPlanItem {
    pub id: Uuid,
    pub meal_plan_id: Uuid,
    pub recipe_id: Uuid,
    pub date: DateTime<Utc>,
    pub servings: i16,
    pub meal_type: MealType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub servings: i16,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Associations, Identifiable)]
#[diesel(table_name = crate::schema::ingridients)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingridient {
    pub id: Uuid,
    pub name: String,
    pub unit: String,
    pub quantity: bigdecimal::BigDecimal,
    pub recipe_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
