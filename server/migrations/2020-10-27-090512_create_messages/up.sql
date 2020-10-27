-- Your SQL goes here
CREATE TABLE messages (
    "id" INT PRIMARY KEY,
    "from" VARCHAR,
    "to" VARCHAR NOT NULL,
    "body" VARCHAR NOT NULL,
    "timestamp" TIMESTAMP WITH TIME ZONE NOT NULL
)