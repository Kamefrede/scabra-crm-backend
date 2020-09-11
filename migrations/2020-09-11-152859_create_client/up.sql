-- Your SQL goes here
CREATE TABLE "client" (
  "id" int4 NOT NULL,
  "name" varchar(255) NOT NULL,
  "address_id" int4 NOT NULL,
  "employees" int4 NOT NULL,
  "client_type" varchar(255) NOT NULL,
  "nif" varchar(255) NOT NULL,
  PRIMARY KEY ("id")
);

ALTER TABLE "client" ADD CONSTRAINT "fk_client_client_1" FOREIGN KEY ("employees") REFERENCES "person" ("id");

