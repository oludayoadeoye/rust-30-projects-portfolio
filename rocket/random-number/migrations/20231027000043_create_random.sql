CREATE TABLE IF NOT EXISTS generated_numbers (
    id SERIAL PRIMARY KEY,
    value INTEGER NOT NULL,
    min_range INTEGER NOT NULL,
    max_range INTEGER NOT NULL,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
