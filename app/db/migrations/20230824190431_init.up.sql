-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    text VARCHAR(255) NOT NULL,
    is_done BOOLEAN NOT NULL DEFAULT FALSE
);