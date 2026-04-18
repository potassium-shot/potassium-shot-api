CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE login_tokens (
    id INTEGER PRIMARY KEY,
    user INTEGER UNIQUE NOT NULL,
    token BLOB UNIQUE NOT NULL,
    expiration INTEGER NOT NULL
);
