mod meal_plan;
pub mod schema;
use std::env;

use chrono::{DateTime, Utc};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use meal_plan::{
    models::Recipe,
    repositories::recipes::{RecipeRepository, RecipeRepositoryInterface},
};
use uuid::{Uuid};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let repo = RecipeRepository {
        db_connection: &mut establish_connection(),
    };

    repo.create(&Recipe {
        id: Uuid::new_v4(),
        name: "Test Recipe".to_string(),
        description: "Test".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        servings: 4,
    });
}
