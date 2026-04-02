CREATE TABLE ar_scenes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE ar_anchors (
    id SERIAL PRIMARY KEY,
    scene_id INTEGER NOT NULL REFERENCES ar_scenes(id) ON DELETE CASCADE,
    spatial_data JSONB NOT NULL,
    label TEXT NOT NULL
);
