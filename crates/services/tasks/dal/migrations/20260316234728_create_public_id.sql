-- Add migration script here
CREATE DOMAIN pub_id AS UUID DEFAULT gen_random_uuid();
