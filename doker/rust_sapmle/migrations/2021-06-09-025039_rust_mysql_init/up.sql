-- Your SQL goes here
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL
);
CREATE TABLE articles
(
    id SERIAL PRIMARY KEY,
    title VARCHAR(64) NOT NULL
);