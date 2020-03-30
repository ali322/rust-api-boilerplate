-- This file should undo anything in `up.sql`
DROP TABLE domains CASCADE;
DROP TABLE roles;
DROP TABLE users;
DROP TABLE actions;
DROP TABLE role_has_actions;
DROP TABLE user_has_roles;