CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL, -- Your SQL goes here
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL
);