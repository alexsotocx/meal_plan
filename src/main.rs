#[macro_use]
extern crate rocket;
mod meal_plan;
pub mod schema;

use meal_plan::{
    db::establish_connection_pool,
    requests::recipes::{create_recipe::create_recipe_handler, get_recipe::get_recipe_handler},
};

#[launch]
fn rocket() -> _ {
    let mut pool = establish_connection_pool();

    rocket::build().manage(pool).mount(
        "/recipes",
        routes![create_recipe_handler, get_recipe_handler],
    )
}
