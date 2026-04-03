-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    public_id pub_id NOT NULL UNIQUE,
    --
    title VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(7) NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'DONE',)),
    ---
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);
CREATE TRIGGER trg_tasks_updated_at BEFORE
UPDATE ON tasks FOR EACH ROW EXECUTE FUNCTION set_updated_at();
-- public_id is queried by API, UNIQUE already creates an index
-- status for filtering by state
CREATE INDEX idx_tasks_status ON tasks (status);
-- created_at for sorting/pagination
CREATE INDEX idx_tasks_created_at ON tasks (created_at);
