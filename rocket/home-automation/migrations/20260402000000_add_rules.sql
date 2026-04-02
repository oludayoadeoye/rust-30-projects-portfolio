CREATE TABLE IF NOT EXISTS rules (
    id SERIAL PRIMARY KEY,
    source_device_id UUID NOT NULL REFERENCES devices(id),
    condition_op TEXT NOT NULL, -- '>', '<', '=='
    threshold TEXT NOT NULL,
    target_device_id UUID NOT NULL REFERENCES devices(id),
    target_state TEXT NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
