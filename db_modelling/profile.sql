ALTER TABLE "profile" DROP CONSTRAINT "fk_profile_profile_1" CASCADE;
ALTER TABLE "profile" DROP CONSTRAINT "fk_profile_profile_2" CASCADE;

DROP TABLE "profile" CASCADE;

CREATE TABLE "profile" (
  "person_id" int4 NOT NULL,
  "displayname" varchar(255),
  "image" varchar(255),
  "phone_number" varchar(255),
  "role" varchar(255),
  "address_id" int4,
  PRIMARY KEY ("person_id")
);

ALTER TABLE "profile" ADD CONSTRAINT "fk_profile_profile_1" FOREIGN KEY ("person_id") REFERENCES "person" ("id");
ALTER TABLE "profile" ADD CONSTRAINT "fk_profile_profile_2" FOREIGN KEY ("address_id") REFERENCES "address" ("id");

