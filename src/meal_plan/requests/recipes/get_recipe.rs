use rocket::{serde::json::Json, State};

use crate::meal_plan::{
    db::DbPool, repositories::recipes::{RecipeRepository, RecipeWithIngridients}, use_cases::get_recipe::get_recipe,
};

#[get("/<id>", format = "json")]
pub async fn get_recipe_handler(pool: &State<DbPool>, id: &str) -> Json<RecipeWithIngridients> {
    Json(get_recipe(
        id.parse().expect("Failed to parse UUID"),
        &mut RecipeRepository {
            db_connection: &mut pool.get().expect("Failed to get DB connection from pool"),
        },
    ))
}
