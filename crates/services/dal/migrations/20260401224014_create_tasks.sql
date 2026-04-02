-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    ---
    title VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(7) NOT NULL,
    ---
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TRIGGER trg_tasks_updated_at BEFORE
UPDATE ON tasks FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE INDEX idx_tasks_status ON tasks (status);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);
