CREATE TABLE interactions (
    id SERIAL PRIMARY KEY,
    user_query TEXT NOT NULL,
    ai_response TEXT NOT NULL,
    confidence_score DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
