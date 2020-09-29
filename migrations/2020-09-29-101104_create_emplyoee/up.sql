-- Your SQL goes here
CREATE TABLE "employee" (
                            "id" int4 NOT NULL,
                            "client_id" int4 NOT NULL,
                            PRIMARY KEY ("id")
);

ALTER TABLE "employee" ADD CONSTRAINT "fk_employee_employee_1" FOREIGN KEY ("id") REFERENCES "person" ("id");
ALTER TABLE "employee" ADD CONSTRAINT "fk_employee_employee_2" FOREIGN KEY ("client_id") REFERENCES "client" ("id");
