-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(50)  NOT NULL,
  password VARCHAR(100)  NOT NULL,
  email VARCHAR(200)  NOT NULL,
  avatar TEXT,
  memo TEXT,
  last_logined_at TIMESTAMP NOT NULL
)