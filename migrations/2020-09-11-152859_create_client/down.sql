-- This file should undo anything in `up.sql`
ALTER TABLE "client" DROP CONSTRAINT "fk_client_client_1" CASCADE;

DROP TABLE "client" CASCADE;

