-- This file should undo anything in `up.sql`
ALTER TABLE "profile" DROP CONSTRAINT "fk_profile_profile_1" CASCADE;
ALTER TABLE "profile" DROP CONSTRAINT "fk_profile_profile_2" CASCADE;

DROP TABLE "profile" CASCADE