-- Your SQL goes here
CREATE TYPE "MealTypeSql" AS ENUM ('breakfast', 'lunch', 'dinner');
CREATE TABLE meal_plan_items (
  id UUID PRIMARY KEY,
  meal_plan_id UUID NOT NULL,
  recipe_id UUID NOT NULL,
  "date" DATE NOT NULL,
  servings SMALLINT NOT NULL,
  meal_type "MealTypeSql" NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);