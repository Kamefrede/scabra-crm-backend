DROP TABLE "address" CASCADE;

CREATE TABLE "address" (
  "id" int4 NOT NULL,
  "name" varchar(255) NOT NULL,
  "line1" varchar(255) NOT NULL,
  "line2" varchar(255),
  "city" varchar(255) NOT NULL,
  "postal_code" varchar(255) NOT NULL,
  "country" varchar(255) NOT NULL,
  "address_type" varchar(255) NOT NULL,
  PRIMARY KEY ("id")
);
COMMENT ON COLUMN "address"."address_type" IS 'Individuals/Headquarters';

