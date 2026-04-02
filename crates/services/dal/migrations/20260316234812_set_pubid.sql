-- Add migration script here
-- public_id: opaque external-facing UUID for all public-facing tables.
-- Never expose the internal `id` (UUIDv7) directly to clients.
CREATE DOMAIN pub_id AS UUID NOT NULL DEFAULT gen_random_uuid();
