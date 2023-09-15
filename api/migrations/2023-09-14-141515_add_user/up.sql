-- Your SQL goes here
CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  picture bytea,
  account_type sender_type NOT NULL
);

CREATE TABLE account_conversation (
  account_id INTEGER REFERENCES account(id),
  conversation_id INTEGER REFERENCES conversation(id),
  PRIMARY KEY(account_id, conversation_id)
)
