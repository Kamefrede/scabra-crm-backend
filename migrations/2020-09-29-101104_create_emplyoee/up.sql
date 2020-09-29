-- Your SQL goes here
CREATE TABLE "employee" (
                            "person_id" int4 NOT NULL,
                            "company_id" int4 NOT NULL,
                            PRIMARY KEY ("person_id")
);

ALTER TABLE "employee" ADD CONSTRAINT "fk_employee_employee_1" FOREIGN KEY ("person_id") REFERENCES "person" ("id");
ALTER TABLE "employee" ADD CONSTRAINT "fk_employee_employee_2" FOREIGN KEY ("company_id") REFERENCES "client" ("id");
