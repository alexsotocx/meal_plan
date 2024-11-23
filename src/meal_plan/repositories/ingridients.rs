use diesel::{PgConnection, RunQueryDsl};

use crate::meal_plan::models::Ingridient;

pub trait IngridientInterface {
    fn create(&mut self, ingridients: &Vec<Ingridient>);
}

pub struct IngridientRepository<'a> {
    pub db_connection: &'a mut PgConnection,
}

impl<'a> IngridientInterface for IngridientRepository<'a> {
    fn create(&mut self, ingridients: &Vec<Ingridient>) {
        use crate::schema::ingridients;

        diesel::insert_into(ingridients::table)
            .values(ingridients)
            .execute(self.db_connection)
            .expect("Error inserting recipe");
    }
}
