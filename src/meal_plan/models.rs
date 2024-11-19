#[derive(Debug)]
pub struct User {
  pub name: String,
  pub password: String,
  pub family_id: u32,
}

#[derive(Debug)]
pub struct Family {
  pub id: u32
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
  Dinner
}

#[derive(Debug)]
pub struct PlanItem {
  pub id: u32,
  pub date: String,
  pub meal_type: MealType,
  pub servings: u8,
  pub recipe_id: u32
}

#[derive(Debug)]
pub struct Recipe {
  pub id: u32,
  pub servings: u8,
  pub description: String
}

#[derive(Debug)]
pub struct Ingridient {
  pub name: String,
  pub unit: String,
  pub quantity: f32
}


