-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  username VARCHAR(50) UNIQUE NOT NULL,
  password VARCHAR(100)  NOT NULL,
  email VARCHAR(200)  NOT NULL,
  avatar TEXT,
  memo TEXT,
  last_logined_at TIMESTAMP NOT NULL
);

CREATE TABLE domains(
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  description VARCHAR(200) NOT NULL
);

CREATE TABLE roles(
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  description VARCHAR(200) NOT NULL,
  domain_id INTEGER NOT NULL,
  FOREIGN KEY(domain_id) REFERENCES domains(id)
);