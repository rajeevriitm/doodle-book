-- Your SQL goes here
CREATE TABLE relationships(
    id SERIAL PRIMARY KEY,
    follower_id INTEGER NOT NULL,
    following_id INTEGER NOT NULL,
    CONSTRAINT unique_followe_following UNIQUE (follower_id,following_id)
);