-- This file should undo anything in `up.sql`
ALTER TABLE "task" DROP CONSTRAINT "fk_task_task_1" CASCADE;
ALTER TABLE "task" DROP CONSTRAINT "fk_task_task_2" CASCADE;

DROP TABLE "task" CASCADE;
