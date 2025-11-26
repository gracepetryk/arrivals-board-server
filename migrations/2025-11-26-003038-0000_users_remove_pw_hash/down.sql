-- This file should undo anything in `up.sql`

ALTER TABLE "users" ADD COLUMN "pw_hash" VARCHAR;

