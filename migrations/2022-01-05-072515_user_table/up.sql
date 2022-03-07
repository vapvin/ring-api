-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    age VARCHAR NOT NULL,
    gender VARCHAR NOT NULL
)