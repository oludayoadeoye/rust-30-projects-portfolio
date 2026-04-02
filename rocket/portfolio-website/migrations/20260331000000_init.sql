CREATE TABLE portfolio_projects (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    tech_stack TEXT[] NOT NULL,
    link TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE portfolio_skills (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    level TEXT NOT NULL
);
