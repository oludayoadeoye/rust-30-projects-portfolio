CREATE TABLE image_analyses (
    id SERIAL PRIMARY KEY,
    image_url TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE image_tags (
    id SERIAL PRIMARY KEY,
    analysis_id INTEGER NOT NULL REFERENCES image_analyses(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    confidence DOUBLE PRECISION NOT NULL
);
