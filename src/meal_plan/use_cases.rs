use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;
use uuid::Uuid;

use super::{
    models::{Ingridient, Recipe},
    repositories::{ingridients::IngridientInterface, recipes::RecipeRepositoryInterface},
};

pub struct IngridientPayload {
    name: String,
    quantity: f32,
    unit: String,
}

pub struct RecipePayload {
    name: String,
    description: String,
    servings: i16,
}

pub struct CreateRecipePayload<'a, 'b, 'c> {
    recipe: RecipePayload,
    ingridients: Vec<&'a IngridientPayload>,
    ingridients_repository: &'b dyn IngridientInterface,
    recipe_repository: &'c dyn RecipeRepositoryInterface,
}

pub struct CreateRecipeResponse {
    recipe: Recipe,
    ingridients: Vec<Ingridient>,
}

pub fn create_recipe(payload: &CreateRecipePayload) -> CreateRecipeResponse {
    let now = Utc::now();
    let recipe = Recipe {
        id: uuid::Uuid::new_v4(),
        name: payload.recipe.name.clone(),
        servings: payload.recipe.servings,
        description: payload.recipe.description.clone(),
        created_at: now,
        updated_at: now,
    };

    let mut recipe_ingridient: Vec<Ingridient> = vec![];

    for ing in &(payload.ingridients) {
        recipe_ingridient.push(Ingridient {
            id: Uuid::new_v4(),
            recipe_id: recipe.id.clone(),
            updated_at: now,
            created_at: now,
            name: ing.name.clone(),
            unit: ing.unit.clone(),
            quantity: BigDecimal::from_f32(ing.quantity).unwrap(),
        });
    }

    payload.recipe_repository.create(&recipe);
    payload.ingridients_repository.create(&recipe_ingridient);

    return CreateRecipeResponse {
        recipe,
        ingridients: recipe_ingridient
    };
}
