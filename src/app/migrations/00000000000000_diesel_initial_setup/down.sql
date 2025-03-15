-- This file was automatically DROPd by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

-- BEGIN TRANSACTION;

-- This file was automatically DROPd by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- DROP TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```

-- BEGIN TRANSACTION;

-- DROP TABLE __diesel_schema_migrations;

DROP TABLE map_location;
DROP TABLE town_location;
DROP TABLE guild;
DROP TABLE bot;

DROP TABLE player_attributes;
DROP INDEX player_nickname;
DROP TABLE player_class;

DROP TABLE item;
DROP TABLE weapon_item;
DROP TABLE gear_item;
DROP TABLE battle_consumable_item;
DROP TABLE non_battle_consumable_item;
DROP TABLE player_inventory;

DROP TABLE battle_participant;
DROP TABLE battle_log;
DROP TABLE battle;
DROP TABLE factions;

DROP TABLE player;

-- COMMIT TRANSACTION;
