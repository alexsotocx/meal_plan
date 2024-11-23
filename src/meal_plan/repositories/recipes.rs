use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::meal_plan::models::Recipe;

pub trait RecipeRepositoryInterface {
    fn create(&mut self, recipe: &Recipe);
}

pub struct RecipeRepository<'a> {
    pub db_connection: &'a mut PgConnection,
}

impl<'a> RecipeRepositoryInterface for RecipeRepository<'a> {
    fn create(&mut self, recipe: &Recipe) {
        use crate::schema::recipes;

        diesel::insert_into(recipes::table)
            .values(recipe)
            .returning(Recipe::as_returning())
            .get_result(self.db_connection)
            .expect("Error inserting recipe");
    }
}
