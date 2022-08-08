-- Your SQL goes here
CREATE TABLE content (
       name Varchar NOT NULL PRIMARY KEY,
       content_inner bytea NOT NULL,
       mime_type Varchar NOT NULL
)
