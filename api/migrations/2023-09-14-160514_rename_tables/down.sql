-- This file should undo anything in `up.sql`
ALTER TABLE Conversation RENAME TO "conversation";
ALTER TABLE Message RENAME TO "message";
ALTER TABLE Account RENAME TO "account";
