CREATE TABLE IF NOT EXISTS migrations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    app_version TEXT NOT NULL,
    sql TEXT NOT NULL
)
