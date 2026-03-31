CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    block_index INTEGER NOT NULL,
    voter_id VARCHAR(255) NOT NULL,
    candidate VARCHAR(255) NOT NULL,
    prev_hash VARCHAR(64) NOT NULL,
    block_hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
