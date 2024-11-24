use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::meal_plan::{models::{Ingridient, Recipe}, repositories::recipes::RecipeRepositoryInterface};


#[derive(Serialize, Deserialize)]
pub struct IngridientPayload {
    pub name: String,
    pub quantity: f32,
    pub unit: String,
}

#[derive(Serialize, Deserialize)]

pub struct RecipePayload {
    pub name: String,
    pub description: String,
    pub servings: i16,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRecipePayload{
    pub recipe: RecipePayload,
    pub ingridients: Vec<IngridientPayload>,
}


#[derive(Serialize)]
pub struct CreateRecipeResponse {
    pub recipe: Recipe,
    pub ingridients: Vec<Ingridient>,
}

pub fn create_recipe(
    payload: &CreateRecipePayload,
    recipe_repository: &mut dyn RecipeRepositoryInterface,
) -> CreateRecipeResponse {
    let now = Utc::now();
    let recipe = Recipe {
        id: uuid::Uuid::new_v4(),
        name: payload.recipe.name.clone(),
        servings: payload.recipe.servings,
        description: payload.recipe.description.clone(),
        created_at: now,
        updated_at: now,
    };

    let mut recipe_ingridient: Vec<Ingridient> = vec![];

    for ing in &(payload.ingridients) {
        recipe_ingridient.push(Ingridient {
            id: Uuid::new_v4(),
            recipe_id: recipe.id.clone(),
            updated_at: now,
            created_at: now,
            name: ing.name.clone(),
            unit: ing.unit.clone(),
            quantity: BigDecimal::from_f32(ing.quantity).unwrap(),
        });
    }

    recipe_repository.create(&recipe, &recipe_ingridient);

    return CreateRecipeResponse {
        recipe,
        ingridients: recipe_ingridient,
    };
}

#[cfg(test)]
mod tests {
    use crate::meal_plan::repositories::recipes::RecipeWithIngridients;

    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        RecipeRepository {}
        impl RecipeRepositoryInterface for RecipeRepository {
            fn create(&mut self, recipe: &Recipe, ingridients: &Vec<Ingridient>);
            fn get(&mut self, id: Uuid) -> RecipeWithIngridients;
        }
    }

    #[test]
    fn test_create_recipe() {
        let mut mock_recipe_repo = MockRecipeRepository::new();


        mock_recipe_repo
            .expect_create()
            .withf(|recipe, ingridients| {
                recipe.name == "Test Recipe" &&
                recipe.servings == 4 &&
                ingridients.len() == 2 &&
                ingridients[0].name == "Sugar" &&
                ingridients[1].name == "Flour"
            })
            .times(1)
            .return_const(());

        let ingridient1 = IngridientPayload {
            name: "Sugar".to_string(),
            quantity: 1.0,
            unit: "cup".to_string(),
        };

        let ingridient2 = IngridientPayload {
            name: "Flour".to_string(),
            quantity: 2.0,
            unit: "cup".to_string(),
        };

        let recipe_payload = RecipePayload {
            name: "Test Recipe".to_string(),
            description: "A test recipe".to_string(),
            servings: 4,
        };

        let payload = CreateRecipePayload {
            recipe: recipe_payload,
            ingridients: vec![ingridient1, ingridient2],
        };

        let response = create_recipe(
            &payload,
            &mut mock_recipe_repo,
        );

        assert_eq!(response.recipe.name, "Test Recipe");
        assert_eq!(response.recipe.servings, 4);
        assert_eq!(response.ingridients.len(), 2);
        assert_eq!(response.ingridients[0].name, "Sugar");
        assert_eq!(response.ingridients[1].name, "Flour");
    }

    #[test]
    fn test_create_recipe_empty_ingridients() {
        let mut mock_recipe_repo = MockRecipeRepository::new();

        mock_recipe_repo
            .expect_create()
            .withf(|recipe, ingridients| {
                recipe.name == "Empty Ingridients Recipe" &&
                recipe.servings == 2 &&
                ingridients.is_empty()
            })
            .times(1)
            .return_const(());
        
        let recipe_payload = RecipePayload {
            name: "Empty Ingridients Recipe".to_string(),
            description: "A recipe with no ingridients".to_string(),
            servings: 2,
        };

        let payload = CreateRecipePayload {
            recipe: recipe_payload,
            ingridients: vec![],
        };

        let response = create_recipe(
            &payload,
            &mut mock_recipe_repo,
        );

        assert_eq!(response.recipe.name, "Empty Ingridients Recipe");
        assert_eq!(response.recipe.servings, 2);
        assert!(response.ingridients.is_empty());
    }
}
