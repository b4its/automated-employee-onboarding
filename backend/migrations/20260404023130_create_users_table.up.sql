-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(50) UNIQUE NOT NULL,
    password TEXT NOT NULL
);