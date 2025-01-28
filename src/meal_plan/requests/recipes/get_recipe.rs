use rocket::{serde::json::Json, State};

use crate::meal_plan::{
    db::DbPool,
    repositories::recipes::{RecipeRepository, RecipeWithIngridients},
    use_cases::get_recipe::get_recipe,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meal_plan::models::Recipe;
    use crate::test_helpers;
    use chrono::Utc;
    use diesel::prelude::*;
    use uuid::Uuid;

    #[async_test]
    async fn test_get_recipe_handler() {
        let pool = test_helpers::connect_db("recipe_test");
        let recipe = Recipe {
            id: Uuid::new_v4(),
            name: "Test recipe".to_string(),
            description: "Test description".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            servings: 4,
        };

        {
            // Release connection client after the block
            let conn = &mut pool.get().expect("Failed to get DB connection from pool");
            diesel::insert_into(crate::schema::recipes::table)
                .values(&recipe)
                .execute(conn)
                .expect("Failed to insert recipe");
        }

        let response = get_recipe_handler(&State::from(&pool), &recipe.id.to_string())
            .await
            .into_inner();
        let recipe_response = response.recipe;
        assert_eq!(recipe_response.id, recipe.id);
        assert_eq!(recipe_response.name, recipe.name);
        assert_eq!(recipe_response.description, recipe.description);
        assert_eq!(recipe_response.servings, recipe.servings);
    }
}
