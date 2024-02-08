CREATE TABLE reminders (
    id UUID PRIMARY KEY,
    body TEXT NOT NULL,
    created_at BIGINT NOT NULL
);