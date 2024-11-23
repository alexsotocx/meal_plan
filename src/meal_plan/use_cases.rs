use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;
use uuid::Uuid;

use super::{
    models::{Ingridient, Recipe},
    repositories::{ingridients::IngridientInterface, recipes::RecipeRepositoryInterface},
};

pub struct IngridientPayload {
    pub name: String,
    pub quantity: f32,
    pub unit: String,
}

pub struct RecipePayload {
    pub name: String,
    pub description: String,
    pub servings: i16,
}

pub struct CreateRecipePayload<'a, 'b, 'c> {
    pub recipe: RecipePayload,
    pub ingridients: Vec<&'a IngridientPayload>,
    pub ingridients_repository: &'b mut dyn IngridientInterface,
    pub recipe_repository: &'c mut dyn RecipeRepositoryInterface,
}

#[derive(Debug)]
pub struct CreateRecipeResponse {
    pub recipe: Recipe,
    pub ingridients: Vec<Ingridient>,
}

pub fn create_recipe(payload: &mut CreateRecipePayload) -> CreateRecipeResponse {
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

    payload.recipe_repository.create(&recipe);
    payload.ingridients_repository.create(&recipe_ingridient);

    return CreateRecipeResponse {
        recipe,
        ingridients: recipe_ingridient,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        IngridientRepository {}
        impl IngridientInterface for IngridientRepository {
            fn create(&mut self, ingridients: &Vec<Ingridient>);
        }
    }

    mock! {
        RecipeRepository {}
        impl RecipeRepositoryInterface for RecipeRepository {
            fn create(&mut self, recipe: &Recipe);
        }
    }

    #[test]
    fn test_create_recipe() {
        let mut mock_ingridient_repo = MockIngridientRepository::new();
        let mut mock_recipe_repo = MockRecipeRepository::new();

        mock_ingridient_repo
            .expect_create()
            .withf(|ingridients| ingridients.len() == 2)
            .times(1)
            .return_const(());

        mock_recipe_repo
            .expect_create()
            .withf(|recipe| recipe.name == "Test Recipe")
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

        let mut payload = CreateRecipePayload {
            recipe: recipe_payload,
            ingridients: vec![&ingridient1, &ingridient2],
            ingridients_repository: &mut mock_ingridient_repo,
            recipe_repository: &mut mock_recipe_repo,
        };

        let response = create_recipe(&mut payload);

        assert_eq!(response.recipe.name, "Test Recipe");
        assert_eq!(response.recipe.servings, 4);
        assert_eq!(response.ingridients.len(), 2);
        assert_eq!(response.ingridients[0].name, "Sugar");
        assert_eq!(response.ingridients[1].name, "Flour");
    }

    #[test]
    fn test_create_recipe_empty_ingridients() {
        let mut mock_ingridient_repo = MockIngridientRepository::new();
        let mut mock_recipe_repo = MockRecipeRepository::new();

        mock_ingridient_repo
            .expect_create()
            .withf(|ingridients| ingridients.is_empty())
            .times(1)
            .return_const(());

        mock_recipe_repo
            .expect_create()
            .withf(|recipe| recipe.name == "Empty Ingridients Recipe")
            .times(1)
            .return_const(());

        let recipe_payload = RecipePayload {
            name: "Empty Ingridients Recipe".to_string(),
            description: "A recipe with no ingridients".to_string(),
            servings: 2,
        };

        let mut payload = CreateRecipePayload {
            recipe: recipe_payload,
            ingridients: vec![],
            ingridients_repository: &mut mock_ingridient_repo,
            recipe_repository: &mut mock_recipe_repo,
        };

        let response = create_recipe(&mut payload);

        assert_eq!(response.recipe.name, "Empty Ingridients Recipe");
        assert_eq!(response.recipe.servings, 2);
        assert!(response.ingridients.is_empty());
    }
}
