CREATE TABLE reminders (
    id UUID PRIMARY KEY,
    content JSONB,
    created_at BIGINT NOT NULL
);