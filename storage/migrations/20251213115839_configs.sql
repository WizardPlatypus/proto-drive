-- Add migration script here
CREATE TABLE configs(
    user_id UUID UNIQUE NOT NULL,
    sorted TEXT,
    ascending BOOLEAN NOT NULL DEFAULT TRUE,
    -- name -- Always visible
    created_at BOOLEAN NOT NULL DEFAULT TRUE,
    edited_at BOOLEAN NOT NULL DEFAULT TRUE,
    owned_by BOOLEAN NOT NULL DEFAULT TRUE,
    edited_by BOOLEAN NOT NULL DEFAULT TRUE,
    filtered BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);