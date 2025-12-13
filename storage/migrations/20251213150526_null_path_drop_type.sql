-- Add migration script here
ALTER TABLE files
ALTER path TYPE TEXT;
ALTER TABLE files
DROP type;