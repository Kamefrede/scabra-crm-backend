-- This file should undo anything in `up.sql`
ALTER TABLE "user" DROP CONSTRAINT "fk_user_user_1" CASCADE;

DROP TABLE "user" CASCADE;