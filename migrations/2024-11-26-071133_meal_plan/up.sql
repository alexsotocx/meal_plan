-- Your SQL goes here
CREATE TABLE meal_plans (
  id UUID PRIMARY KEY,
  "week" SMALLINT NOT NULL,
  "year" SMALLINT NOT NULL,
  created_at TIMESTAMP  WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP  WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);