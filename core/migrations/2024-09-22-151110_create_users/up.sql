CREATE TABLE users (
    email VARCHAR PRIMARY KEY,
    hashed_password VARCHAR,
    display_name VARCHAR NOT NULL
)