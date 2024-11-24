use rocket::{serde::json::Json, State};

use crate::meal_plan::{
    db::DbPool, repositories::recipes, use_cases::create_recipe::{create_recipe, CreateRecipePayload, CreateRecipeResponse}, 
};

#[post("/", format = "json", data = "<payload>")]
pub async fn create_recipe_handler(
    pool: &State<DbPool>,
    payload: Json<CreateRecipePayload>,
) -> Json<CreateRecipeResponse>{
    
    Json(create_recipe(
        &payload.into_inner(),
        &mut recipes::RecipeRepository {
            db_connection: &mut pool.get().expect("Failed to get DB connection from pool"),
        },
    ))
}
