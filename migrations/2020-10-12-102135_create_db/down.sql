-- This file should undo anything in `up.sql`
ALTER TABLE "person" DROP CONSTRAINT "fk_person_person_1" CASCADE;
ALTER TABLE "person" DROP CONSTRAINT "fk_person_person_2" CASCADE;
ALTER TABLE "task" DROP CONSTRAINT "fk_task_task_1" CASCADE;
ALTER TABLE "task" DROP CONSTRAINT "fk_task_task_2" CASCADE;
ALTER TABLE "user" DROP CONSTRAINT "fk_user_user_1" CASCADE;
ALTER TABLE "user_auth_token" DROP CONSTRAINT "fk_user_auth_token_user_auth_token_1" CASCADE;

ALTER TABLE "address" DROP CONSTRAINT "address_pkey" CASCADE;
ALTER TABLE "client" DROP CONSTRAINT "client_pkey" CASCADE;
ALTER TABLE "person" DROP CONSTRAINT "person_pkey" CASCADE;
ALTER TABLE "user" DROP CONSTRAINT "user_pkey" CASCADE;
ALTER TABLE "user_auth_token" DROP CONSTRAINT "user_auth_token_pkey" CASCADE;

DROP TABLE "address" CASCADE;
DROP TABLE "client" CASCADE;
DROP TABLE "person" CASCADE;
DROP TABLE "task" CASCADE;
DROP TABLE "user" CASCADE;
DROP TABLE "user_auth_token" CASCADE;