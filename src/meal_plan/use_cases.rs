use super::{db::Connection, models::{Ingridient, Recipe}};

 
struct IngridientPayload {
    name: String,
    quantity: f32,
    unit: String,
}
pub struct CreateRecipePayload<'a> {
    name: String,
    servings: u8,
    ingridients: Vec<&'a IngridientPayload>
}

pub struct CreateRecipeResponse {
    recipe: Recipe,
    ingridients: Vec<Ingridient>
}

pub fn create_recipe(payload: &CreateRecipePayload, db:  &dyn Connection) -> CreateRecipeResponse {
    let q = format!("INSERT INTO recipes(id, name, services) VALUES ({}, {}, {})", 1, payload.name, payload.servings);
    let mut ingridients_array = vec![];
    db.execute(q).unwrap();
    for p in &payload.ingridients {
        let q = format!("INSERT INTO ingridients(id, name, quantity, unit) VALUES ({}, {}, {}, {})", 1, p.name, p.quantity, p.unit);
        db.execute(q).unwrap();
        ingridients_array.push(Ingridient {
            name: p.name.clone(),
            quantity: p.quantity,
            unit: p.unit.clone()
        });
    }

    return CreateRecipeResponse {
        recipe: Recipe {
            id: 1,
            servings: payload.servings,
            description: payload.name.clone()
        },
        ingridients: ingridients_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Error;

    struct MockConnection {
    }

    impl Connection for MockConnection {
        fn execute(&self, q: String) -> Result<String, Error> {
            return Ok(q)
        }
    }

    #[test]
    fn test_create_recipe() {
        let ingridient = IngridientPayload {
            name: "test".to_string(),
            quantity: 1.0,
            unit: "kg".to_string()
        };
        let payload = CreateRecipePayload {
            name: "test".to_string(),
            servings: 1,
            ingridients: vec![&ingridient]
        };
        let db = MockConnection {};
        let response = create_recipe(&payload, &db);
        assert_eq!(response.recipe.id, 1);
        assert_eq!(response.recipe.servings, 1);
        assert_eq!(response.recipe.description, "test");
        assert_eq!(response.ingridients.len(), 1);
        assert_eq!(response.ingridients[0].name, "test");
        assert_eq!(response.ingridients[0].quantity, 1.0);
        assert_eq!(response.ingridients[0].unit, "kg");
    }
}