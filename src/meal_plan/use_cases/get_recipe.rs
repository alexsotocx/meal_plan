use uuid::Uuid;

use crate::meal_plan::repositories::recipes::{RecipeRepositoryInterface, RecipeWithIngridients};

pub fn get_recipe(
    recipe_id: Uuid,
    recipe_repository: &mut dyn RecipeRepositoryInterface,
) -> RecipeWithIngridients {
    recipe_repository.get(recipe_id)
}
