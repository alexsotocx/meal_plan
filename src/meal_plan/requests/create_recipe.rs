use rocket::{serde::json::Json, State};

use crate::meal_plan::{
    db::DbPool, repositories::{ingridients, recipes}, use_cases::{self, CreateRecipePayload, CreateRecipeResponse}
};

#[post("/recipe", format = "json", data = "<payload>")]
pub async fn create_recipe_handler(
    pool: &State<DbPool>,
    payload: Json<CreateRecipePayload>,
) -> Json<CreateRecipeResponse>{
    Json(use_cases::create_recipe(
        &payload.into_inner(),
        &mut ingridients::IngridientRepository {
            db_connection: &mut pool.get().expect("Failed to get DB connection from pool"),
        },
        &mut recipes::RecipeRepository {
            db_connection: &mut pool.get().expect("Failed to get DB connection from pool"),
        },
    ))
}
