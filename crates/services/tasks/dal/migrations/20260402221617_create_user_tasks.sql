-- Add migration script here
CREATE TABLE IF NOT EXISTS user_tasks (
    task_id UUID NOT NULL REFERENCES tasks(id) ON UPDATE CASCADE ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
    ---
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    --
    PRIMARY KEY (user_id, task_id)
);
CREATE TRIGGER trg_user_tasks_updated_at BEFORE
UPDATE ON user_tasks FOR EACH ROW EXECUTE FUNCTION set_updated_at();
-- PK (user_id, task_id) covers: "all tasks for a user"
-- this covers the reverse: "all users for a task"
CREATE INDEX idx_user_tasks_task_id ON user_tasks (task_id);
