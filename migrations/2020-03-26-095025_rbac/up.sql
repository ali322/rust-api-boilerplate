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

CREATE TABLE actions(
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  description VARCHAR(200) NOT NULL,
  domain_id INTEGER NOT NULL,
  FOREIGN KEY(domain_id) REFERENCES domains(id)
);

CREATE TABLE user_has_roles(
  user_id UUID,
  role_id INTEGER,
  expire TIMESTAMP NOT NULL,
  PRIMARY KEY(user_id, role_id)
);

CREATE TABLE role_has_actions(
  role_id INTEGER,
  action_id INTEGER,
  PRIMARY KEY(action_id, role_id)
);