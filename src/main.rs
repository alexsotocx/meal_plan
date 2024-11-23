mod meal_plan;
pub mod schema;
use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use meal_plan::{repositories::{ingridients::IngridientRepository, recipes::RecipeRepository}, use_cases::{create_recipe, IngridientPayload, RecipePayload}};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut db_connection = establish_connection();
    let mut db_connection2 = establish_connection();
    let mut recipe_repository = RecipeRepository {
        db_connection: &mut db_connection,
    };
    let mut ingridient_repository = IngridientRepository {
        db_connection: &mut db_connection2,
    };

    let binding1 = IngridientPayload {
                name: "Pasta".to_string(),
                unit: "g".to_string(),
                quantity: 500.0,
            };
    let binding2 = IngridientPayload {
                name: "Tomato sauce".to_string(),
                unit: "g".to_string(),
                quantity: 200.0,
            };
    let mut create_recipe_payload = meal_plan::use_cases::CreateRecipePayload {
        recipe: RecipePayload {
            name: "Pasta".to_string(),
            servings: 4,
            description: "Pasta with tomato sauce".to_string(),
        },
        ingridients: vec![
            &binding1,
            &binding2,
        ],
        recipe_repository: &mut recipe_repository,
        ingridients_repository: &mut ingridient_repository,
    };

    let result =  create_recipe(&mut create_recipe_payload);

    println!("Recipe {:?}", result);

}
