-- Your SQL goes here
CREATE TABLE meal_plan_items (
  id UUID PRIMARY KEY,
  meal_plan_id UUID NOT NULL,
  recipe_id UUID NOT NULL,
  "date" DATE NOT NULL,
  servings SMALLINT NOT NULL,
  meal_type SMALLINT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);