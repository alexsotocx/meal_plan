use super::{db::{ Connection}, models::Recipe};

 
struct IngridientPayload {
    name: String,
    quantity: u8,
    unit: String,
}
pub struct CreateRecipePayload<'a> {
    name: String,
    servings: u8,
    ingridients: Vec<&'a IngridientPayload>
}

pub fn create_recipe(payload: &CreateRecipePayload, db: &dyn Connection) -> &Recipe {
    let q = format!("INSERT INTO recipes(id, name, services) VALUES ({}, {}, {})", 1, payload.name, payload.servings);
    db.execute(q);
    payload.ingridients
}