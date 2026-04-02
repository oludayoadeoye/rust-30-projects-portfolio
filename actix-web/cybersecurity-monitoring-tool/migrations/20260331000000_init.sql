CREATE TABLE alerts (
    id SERIAL PRIMARY KEY,
    severity TEXT NOT NULL,
    message TEXT NOT NULL,
    source_ip TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
