CREATE TABLE nlp_analyses (
    id SERIAL PRIMARY KEY,
    input_text TEXT NOT NULL,
    sentiment TEXT NOT NULL,
    language TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
