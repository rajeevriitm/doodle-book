-- Your SQL goes here
CREATE TABLE drawings(
    id SERIAL PRIMARY KEY,
    points TEXT NOT NULL DEFAULT '[[()]]',
    width INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id INTEGER NOT NULL REFERENCES users (id)
);
SELECT diesel_manage_updated_at('drawings');