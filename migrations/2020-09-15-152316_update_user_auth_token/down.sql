-- This file should undo anything in `up.sql`
ALTER TABLE "user_auth_token" DROP CONSTRAINT "fk_user_auth_token_user_auth_token_1" CASCADE;

DROP TABLE "user_auth_token" CASCADE;
