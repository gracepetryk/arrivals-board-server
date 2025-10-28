-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

create table "user"
(
    id      uuid    not null
        constraint user_pk
            primary key,
    email   varchar not null,
    pw_hash varchar
);

alter table "user"
    owner to postgres;

create unique index user_email_uindex
    on "user" (email);

create table public.tracking_info
(
    id          uuid not null
        constraint tracking_info_pk
            primary key,
    user_id     uuid not null
        constraint tracking_info_user_id_fk
            references public."user",
    min_lat     numeric,
    min_long    numeric,
    max_lat     numeric,
    max_long    numeric,
    origin_iata char(3),
    dest_iata   char(3),
    updated_at  timestamp
);

alter table public.tracking_info
    owner to postgres;
