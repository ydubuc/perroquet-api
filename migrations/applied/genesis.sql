CREATE TABLE users(
    id UUID PRIMARY KEY,
    id_apple TEXT,
    username TEXT NOT NULL,
    username_key TEXT UNIQUE NOT NULL,
    email TEXT NOT NULL,
    email_key TEXT UNIQUE NOT NULL,
    email_pending TEXT,
    password TEXT,
    displayname TEXT NOT NULL,
    avatar_url TEXT,
    updated_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);

CREATE TABLE devices(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token TEXT UNIQUE,
    messaging_token TEXT,
    refreshed_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);

CREATE TABLE memos(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    priority SMALLINT NOT NULL,
    status TEXT NOT NULL,
    visibility SMALLINT NOT NULL,
    frequency TEXT,
    trigger_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);
