use diesel::{
    query_dsl::methods::FindDsl, BelongingToDsl, PgConnection, RunQueryDsl, SelectableHelper,
};
use serde::Serialize;
use uuid::Uuid;

use crate::meal_plan::models::{Ingridient, Recipe};

#[derive(Serialize)]
pub struct RecipeWithIngridients {
    pub recipe: Recipe,
    pub ingridients: Vec<Ingridient>,
}
pub trait RecipeRepositoryInterface {
    fn create(&mut self, recipe: &Recipe, ingridients: &Vec<Ingridient>);
    fn get(&mut self, id: Uuid) -> RecipeWithIngridients;
}

pub struct RecipeRepository<'a> {
    pub db_connection: &'a mut PgConnection,
}

impl<'a> RecipeRepositoryInterface for RecipeRepository<'a> {
    fn create(&mut self, recipe: &Recipe, ingridients: &Vec<Ingridient>) {
        use crate::schema::ingridients;
        use crate::schema::recipes;

        self.db_connection
            .build_transaction()
            .run(|connection| {
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
            })
            .unwrap();
    }

    fn get(&mut self, id: Uuid) -> RecipeWithIngridients {
        use crate::schema::recipes::dsl;

        let recipe = dsl::recipes
            .find(id)
            .get_result::<Recipe>(self.db_connection)
            .unwrap();

        let ingridients = Ingridient::belonging_to(&recipe)
            .get_results::<Ingridient>(self.db_connection)
            .unwrap();

        return RecipeWithIngridients {
            ingridients,
            recipe,
        };
    }
}
