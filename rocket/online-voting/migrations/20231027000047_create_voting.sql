CREATE TABLE IF NOT EXISTS polls (
    id SERIAL PRIMARY KEY,
    question TEXT NOT NULL,
    options JSONB NOT NULL, -- e.g. ["Option A", "Option B"]
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS votes (
    id SERIAL PRIMARY KEY,
    poll_id INTEGER REFERENCES polls(id) ON DELETE CASCADE,
    voter_id UUID NOT NULL,
    selected_option TEXT NOT NULL,
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
