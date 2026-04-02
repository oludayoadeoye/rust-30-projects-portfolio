CREATE TABLE IF NOT EXISTS conversions (
    id SERIAL PRIMARY KEY,
    input_unit VARCHAR(50) NOT NULL,
    input_value FLOAT NOT NULL,
    output_unit VARCHAR(50) NOT NULL,
    output_value FLOAT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
