ALTER TABLE "user" DROP CONSTRAINT "fk_user_user_1" CASCADE;

DROP TABLE "user" CASCADE;

CREATE TABLE "user" (
  "id" int4 NOT NULL,
  "person_id" int4,
  "username" varchar(255) NOT NULL,
  "email" varchar(255) NOT NULL,
  "hashed_password" varchar(255) NOT NULL,
  PRIMARY KEY ("id")
);

ALTER TABLE "user" ADD CONSTRAINT "fk_user_user_1" FOREIGN KEY ("person_id") REFERENCES "person" ("id");

