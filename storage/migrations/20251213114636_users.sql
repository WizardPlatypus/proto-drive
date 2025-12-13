-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    login TEXT UNIQUE NOT NULL,
    phc TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);