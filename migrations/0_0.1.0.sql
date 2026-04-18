CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE login_tokens (
    id INTEGER PRIMARY KEY,
    user INTEGER NOT NULL,
    token TEXT NOT NULL,
    expiration INTEGER NOT NULL
);
