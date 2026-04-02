-- Add migration script here
-- Reusable trigger function for auto-updating updated_at columns.
-- Applied to any table that has an updated_at column.
CREATE OR REPLACE FUNCTION set_updated_at () returns trigger AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ language plpgsql;
