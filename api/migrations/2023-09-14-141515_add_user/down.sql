-- This file should undo anything in `up.sql`

-- Remove the foreign key constraints first
-- Drop the foreign key constraint for customer_id
ALTER TABLE conversation
DROP CONSTRAINT IF EXISTS conversation_customer_id_fkey;

-- Drop the foreign key constraint for service_provider_id
ALTER TABLE conversation
DROP CONSTRAINT IF EXISTS conversation_service_provider_id_fkey;

-- Remove the columns
ALTER TABLE conversation
DROP COLUMN IF EXISTS customer_id;

ALTER TABLE conversation
DROP COLUMN IF EXISTS service_provider_id;

DROP TABLE account;
