-- Your SQL goes here
CREATE TABLE posts (
   id SERIAL PRIMARY KEY,
   title VARCHAR(100) NOT NULL,
   body TEXT NOT NULL,
   published BOOLEAN NOT NULL DEFAULT '0'
)