CREATE TABLE IF NOT EXISTS bill_records (
    id SERIAL PRIMARY KEY,
    total_bill FLOAT NOT NULL,
    tip_percentage FLOAT NOT NULL,
    num_people INTEGER NOT NULL,
    tip_amount FLOAT NOT NULL,
    total_per_person FLOAT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
