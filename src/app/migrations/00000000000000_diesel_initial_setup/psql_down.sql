-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

-- BEGIN TRANSACTION;

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass) CASCADE;
DROP FUNCTION IF EXISTS diesel_set_updated_at() CASCADE;

DROP SCHEMA IF EXISTS "warhundred" CASCADE;
DROP FUNCTION IF EXISTS read_only CASCADE;

-- COMMIT TRANSACTION;
