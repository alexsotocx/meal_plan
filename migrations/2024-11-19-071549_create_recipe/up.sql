CREATE TABLE
    recipes (
        id UUID PRIMARY KEY,
        name VARCHAR(256) NOT NULL,
        description TEXT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL,
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL
    );