CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       name VARCHAR NOT NULL,
       pwd VARCHAR NOT NULL,
       privs INT NOT NULL
)
