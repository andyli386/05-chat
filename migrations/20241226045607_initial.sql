-- Add migration script here
-- create user table
CREATE TABLE IF NOT EXISTS users (
  id bigserial PRIMARY KEY,
  fullname varchar(64) NOT NULL,
  email varchar(64) NOT NULL,
  password varchar(64) NOT NULL,
  create_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users(email);

CREATE TYPE chat_type AS ENUM(
  'single',
  'group',
  'private_channel',
  'public_channel'
);

CREATE TABLE IF NOT EXISTS chats (
  id bigserial PRIMARY KEY,
  name varchar(128) NOT NULL UNIQUE,
  type chat_type NOT NULL,
  -- user id list
  members bigint[] NOT NULL,
  create_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages (
  id bigserial PRIMARY KEY,
  chat_id bigint NOT NULL REFERENCES chats(id),
  sender_id bigint NOT NULL REFERENCES users(id),
  content text NOT NULL,
  images text[],
  create_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS chat_id_created_at_index ON messages(chat_id, create_at DESC);

CREATE INDEX IF NOT EXISTS sender_id_index ON messages(sender_id, create_at DESC);
