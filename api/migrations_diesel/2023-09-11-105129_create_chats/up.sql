-- Your SQL goes here
CREATE TYPE message_type AS ENUM ('quote_offer', 'reject_quote_message', 'standard_message', 'accept_quote_message');
CREATE TYPE sender_type AS ENUM ('customer', 'service_provider');
CREATE TYPE conversation_state AS ENUM ('quoted', 'rejected', 'accepted');

CREATE TABLE conversation (
  id SERIAL PRIMARY KEY,
  customer_name VARCHAR(255) NOT NULL,
  service_provider_name VARCHAR(255) NOT NULL,
  state conversation_state NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE message (
  id SERIAL PRIMARY KEY,
  conversation_id INT NOT NULL,
  message_type message_type NOT NULL,
  text TEXT,
  sender_type sender_type NOT NULL,
  read_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  FOREIGN KEY (conversation_id) REFERENCES conversation(id)
);
