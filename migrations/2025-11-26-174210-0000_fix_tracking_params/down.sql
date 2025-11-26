-- This file should undo anything in `up.sql`
ALTER TABLE "tracking_params" DROP COLUMN "min_lat";
ALTER TABLE "tracking_params" DROP COLUMN "min_long";
ALTER TABLE "tracking_params" DROP COLUMN "max_lat";
ALTER TABLE "tracking_params" DROP COLUMN "max_long";
ALTER TABLE "tracking_params" ADD COLUMN "min_lat" FLOAT8;
ALTER TABLE "tracking_params" ADD COLUMN "min_long" FLOAT8;
ALTER TABLE "tracking_params" ADD COLUMN "max_lat" FLOAT8;
ALTER TABLE "tracking_params" ADD COLUMN "max_long" FLOAT8;


