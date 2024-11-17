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
  week: u8,
}

#[derive(Debug)]
pub enum MealType {
  Breakfast,
  Lunch,
  Dinner
}

#[derive(Debug)]
pub struct PlanItem {
  date: String,
  meal_type: MealType,
  servings: u8,
  recipe_id: u32
}

#[derive(Debug)]
pub struct Recipe {
  servings: u8,
  description: String
}

#[derive(Debug)]
pub struct Ingridient {
  name: String,
  unit: String,
  quantity: f32
}


