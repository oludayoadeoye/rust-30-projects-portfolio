CREATE TABLE recordings (
    id SERIAL PRIMARY KEY,
    transcript TEXT NOT NULL,
    language TEXT NOT NULL,
    duration_seconds DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
