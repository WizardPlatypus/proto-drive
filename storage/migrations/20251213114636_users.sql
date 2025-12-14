-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    login TEXT UNIQUE NOT NULL,
    phc TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);