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
    use chrono::Utc;
    use diesel::prelude::*;
    use diesel::r2d2::ConnectionManager;
    use diesel::sql_query;
    use diesel::PgConnection;
    use diesel::{connection::SimpleConnection, r2d2::Pool};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use rocket::http::hyper::server::conn;
    use uuid::Uuid;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    pub fn connect_db(test_db: &str) -> Pool<ConnectionManager<PgConnection>> {
        let pool = crate::meal_plan::db::establish_connection_pool(
            "postgres://postgres:postgres@db".to_string(),
            1,
        );
        let conn = &mut pool.get().expect("Failed to get DB connection from pool");
        sql_query(format!("DROP DATABASE IF EXISTS {}", test_db).as_str())
            .execute(conn)
            .expect("Error deleting test database");

        sql_query(format!("CREATE DATABASE {}", test_db).as_str())
            .execute(conn)
            .expect("Error creating test database");

        let pool = crate::meal_plan::db::establish_connection_pool(
            format!("postgres://postgres:postgres@db/{}", test_db),
            1,
        );

        let conn = &mut pool.get().expect("Failed to get DB connection from pool");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        return pool;
    }

    #[async_test]
    async fn test_get_recipe_handler() {
        let pool = connect_db("recipe_test");
        let conn = &mut pool.get().expect("Failed to get DB connection from pool");

        conn.begin_test_transaction().unwrap();
        let recipe = Recipe {
            id: Uuid::new_v4(),
            name: "Test recipe".to_string(),
            description: "Test description".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            servings: 4,
        };

        diesel::insert_into(crate::schema::recipes::table)
            .values(&recipe)
            .execute(conn)
            .expect("Failed to insert recipe");

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
