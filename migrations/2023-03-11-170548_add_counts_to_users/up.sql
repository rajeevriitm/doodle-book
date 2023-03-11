-- Your SQL goes here
ALTER TABLE users
    ADD followers_count INTEGER NOT NULL DEFAULT 0,
    ADD following_count INTEGER NOT NULL DEFAULT 0;