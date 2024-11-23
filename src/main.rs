mod meal_plan;
pub mod schema;
use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut db_connection = establish_connection();


}
