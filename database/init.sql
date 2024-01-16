CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    salt VARCHAR(50) NOT NULL UNIQUE, 
    hashed_password VARCHAR(255) NOT NULL,
    refresh_token TEXT,
    email VARCHAR(255) NOT NULL UNIQUE
);

