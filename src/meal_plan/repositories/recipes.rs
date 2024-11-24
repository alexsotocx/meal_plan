use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::meal_plan::models::{Ingridient, Recipe};

pub trait RecipeRepositoryInterface {
    fn create(&mut self, recipe: &Recipe, ingridients: &Vec<Ingridient>);
}

pub struct RecipeRepository<'a> {
    pub db_connection: &'a mut PgConnection,
}

impl<'a> RecipeRepositoryInterface for RecipeRepository<'a> {
    fn create(&mut self, recipe: &Recipe, ingridients: &Vec<Ingridient>) {
        use crate::schema::recipes;
        use crate::schema::ingridients;

        self.db_connection.build_transaction().run( |connection| {
            diesel::insert_into(recipes::table)
                .values(recipe)
                .returning(Recipe::as_returning())
                .get_result(connection)
                .expect("Error inserting recipe");

                diesel::insert_into(ingridients::table)
                    .values(ingridients)
                    .execute(connection)
                    .expect("Error inserting ingridient");


            diesel::QueryResult::Ok(())
        });
    }
}
