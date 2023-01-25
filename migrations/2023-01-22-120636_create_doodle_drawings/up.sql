-- Your SQL goes here
CREATE TABLE drawings(
    id SERIAL PRIMARY KEY,
    points TEXT NOT NULL DEFAULT '[[()]]',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('drawings');