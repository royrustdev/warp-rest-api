CREATE TABLE IF NOT EXISTS accounts (
    id serial NOT NULL,
    email VARCHAR(255) NOT NULL PRIMARY KEY,
    PASSWORD VARCHAR(255) NOT NULL
);