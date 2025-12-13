-- Add migration script here
CREATE TABLE files(
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT,
    path TEXT UNIQUE NOT NULL,
    owned_by UUID NOT NULL,
    edited_by UUID,
    deleted_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    edited_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (owned_by) REFERENCES users(id),
    FOREIGN KEY (edited_by) REFERENCES users(id),
    FOREIGN KEY (deleted_by) REFERENCES users(id)
);