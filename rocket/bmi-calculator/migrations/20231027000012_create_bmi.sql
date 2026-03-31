CREATE TABLE IF NOT EXISTS bmi_records (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR(255) NOT NULL,
    height_cm FLOAT NOT NULL,
    weight_kg FLOAT NOT NULL,
    bmi FLOAT NOT NULL,
    category VARCHAR(50) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
