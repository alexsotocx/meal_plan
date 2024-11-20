use diesel::{connection, Connection, PgConnection};

use crate::meal_plan::models::Recipe;

pub trait RecipeRepositoryInterface {
    fn create(recipe: &Recipe);
}

pub struct RecipeRepository<'a> {
    db_connection: &'a PgConnection
}

impl<'a> RecipeRepositoryInterface for RecipeRepository<'a> {
    fn create(recipe: &Recipe) {
        
    }
}