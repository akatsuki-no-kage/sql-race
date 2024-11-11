CREATE TABLE IF NOT EXISTS test (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    score INT NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);

INSERT INTO test (username, score) VALUES ('hung', 39);
