#[macro_use]
extern crate rocket;
mod meal_plan;
pub mod schema;

use std::env;

use dotenvy::dotenv;
use meal_plan::{
    db::establish_connection_pool,
    requests::recipes::{create_recipe::create_recipe_handler, get_recipe::get_recipe_handler},
};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_size: u32 = env::var("DATABASE_POOL_SIZE").unwrap_or(10.to_string()).parse().unwrap();
    let mut pool = establish_connection_pool(database_url, pool_size);

    rocket::build().manage(pool).mount(
        "/recipes",
        routes![create_recipe_handler, get_recipe_handler],
    )
}
