-- Add migration script here
ALTER TABLE users
ALTER id SET DEFAULT gen_random_uuid();
ALTER TABLE files
ALTER id SET DEFAULT gen_random_uuid();
