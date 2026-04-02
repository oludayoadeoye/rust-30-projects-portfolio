CREATE TABLE IF NOT EXISTS temp_conversions (
    id SERIAL PRIMARY KEY,
    input_unit VARCHAR(10) NOT NULL,
    input_value FLOAT NOT NULL,
    output_unit VARCHAR(10) NOT NULL,
    output_value FLOAT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
