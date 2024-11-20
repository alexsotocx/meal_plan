use diesel::prelude::*;

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
    pub id: String,
    pub servings: u8,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct Ingridient {
    pub id: String,
    pub recipe_id: String,
    pub name: String,
    pub unit: String,
    pub quantity: f32,
    pub created_at: String,
    pub updated_at: String,
}
