-- Your SQL goes here
CREATE TABLE ingridients (
    id UUID PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    unit VARCHAR(256) NOT NULL,
    quantity DECIMAL NOT NULL,
    recipe_id UUID REFERENCES recipes(id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);