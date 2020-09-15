-- Your SQL goes here

CREATE TABLE "user_auth_token" (
  "user_id" int4 NOT NULL,
  "login_session" varchar(255) NOT NULL,
  "generated_at" timestamptz(255) NOT NULL,
  "expires_at" timestamptz(255) NOT NULL,
  PRIMARY KEY ("user_id")
);

ALTER TABLE "user_auth_token" ADD CONSTRAINT "fk_user_auth_token_user_auth_token_1" FOREIGN KEY ("user_id") REFERENCES "user" ("id");