-- Your SQL goes here
CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  picture bytea,
  account_type sender_type NOT NULL
);

ALTER TABLE conversation
ADD COLUMN customer_id INTEGER REFERENCES account(id);

ALTER TABLE conversation
ADD COLUMN service_provider_id INTEGER REFERENCES account(id);
