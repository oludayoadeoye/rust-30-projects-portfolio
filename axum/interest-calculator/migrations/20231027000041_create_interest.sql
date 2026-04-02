CREATE TABLE IF NOT EXISTS interest_calculations (
    id SERIAL PRIMARY KEY,
    principal DECIMAL(12,2) NOT NULL,
    rate FLOAT NOT NULL,
    time_years FLOAT NOT NULL,
    compound_frequency INTEGER DEFAULT 1,
    total_amount DECIMAL(12,2) NOT NULL,
    interest_earned DECIMAL(12,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
