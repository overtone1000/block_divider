CREATE TABLE users (
    email TEXT PRIMARY KEY NOT NULL,
    hashed_password VARCHAR,
    display_name VARCHAR NOT NULL
)