-- Your SQL goes here
CREATE TABLE "users"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"first_name" VARCHAR NOT NULL,
	"last_name" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"timestamp" TIMESTAMP NOT NULL
);

