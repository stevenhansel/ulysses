-- Create the initial profiles table for proxy model management.
CREATE TABLE IF NOT EXISTS profiles (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    backend_url TEXT NOT NULL,
    active      INTEGER NOT NULL DEFAULT 0
);
