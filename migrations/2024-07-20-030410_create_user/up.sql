-- Your SQL goes here


CREATE TABLE users (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR NOT NULL,
    address VARCHAR(255) NOT NULL
);

CREATE INDEX index_users_on_email ON users (email);

