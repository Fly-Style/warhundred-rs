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

-- BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE;

CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS
$$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS
$$
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

CREATE SCHEMA IF NOT EXISTS warhundred;

--- MAP OBJECTS ---

CREATE TABLE IF NOT EXISTS map_location
(
    id                  SERIAL PRIMARY KEY,
    "name"              TEXT  NOT NULL,
    "location"          POINT NOT NULL,
    location_difficulty INT   NOT NULL DEFAULT 1,
    movement_accel      FLOAT NOT NULL DEFAULT 1,
    aggression_prob     FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS town_location
(
    id     SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    UNIQUE ("name")
);

--- GUILDS ---

CREATE TABLE IF NOT EXISTS guild
(
    id     SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    rank   INT  NOT NULL DEFAULT 1,
    UNIQUE ("name")
);

--- PLAYERS ---

CREATE TABLE IF NOT EXISTS player
(
    id                 BIGSERIAL PRIMARY KEY,
    nickname           TEXT      NOT NULL,
    email              TEXT      NOT NULL,
    password           TEXT      NOT NULL,
    last_login         TIMESTAMP NOT NULL DEFAULT NOW(),
    last_map_location  INT       NOT NULL,
    last_town_location INT       NOT NULL,
    guild_id           INT,
--     last_map_location  INT       NOT NULL REFERENCES map_location (id),
--     last_town_location INT REFERENCES town_location (id),
--     guild_id           INT REFERENCES guild (id),
    UNIQUE (nickname, email)
);

CREATE UNIQUE INDEX IF NOT EXISTS player_nickname ON player (nickname);

CREATE TABLE IF NOT EXISTS player_class
(
    class_id   INT         NOT NULL PRIMARY KEY,
    class_name VARCHAR(16) NOT NULL
);

INSERT INTO player_class
VALUES
    (0, 'no-class'),
    (1, 'warrior'),
    (2, 'archer'),
    (3, 'healer'),
    (4, 'rogue'),
    (5, 'lancer');

CREATE TABLE IF NOT EXISTS player_attributes
(
    id         BIGSERIAL PRIMARY KEY,
    class_id   INT         NOT NULL REFERENCES player_class (class_id),
    player_id  BIGINT      NOT NULL REFERENCES player (id),
    strength   INT         NOT NULL DEFAULT 0,
    dexterity  INT         NOT NULL DEFAULT 0,
    physique   INT         NOT NULL DEFAULT 0,
    luck       INT         NOT NULL DEFAULT 0,
    intellect  INT         NOT NULL DEFAULT 0,
    experience BIGINT      NOT NULL,
    level      INT         NOT NULL DEFAULT 0,
    valor      INT         NOT NULL DEFAULT 0,
    rank       VARCHAR(32) NOT NULL DEFAULT 0
);

--- ITEMS ---

CREATE TABLE IF NOT EXISTS item
(
    id            SERIAL PRIMARY KEY,
    name          TEXT NOT NULL,
    level_req     INT  NOT NULL DEFAULT 0,
    strength_req  INT  NOT NULL DEFAULT 0,
    dexterity_req INT  NOT NULL DEFAULT 0,
    physique_req  INT  NOT NULL DEFAULT 0,
    intellect_req INT  NOT NULL DEFAULT 0,
    valor_req     INT  NOT NULL DEFAULT 0,
    class_req     INT REFERENCES player_class (class_id)
);

CREATE TABLE IF NOT EXISTS weapon_item
(
    id                   SERIAL PRIMARY KEY,
    item_id              INT NOT NULL REFERENCES item (id),
    action_points_to_use INT NOT NULL DEFAULT 2,
    basic_damage         INT NOT NULL DEFAULT 0,
    "range"              INT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS gear_item
(
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES item (id)
);

CREATE TABLE IF NOT EXISTS battle_consumable_item
(
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES item (id)
);

CREATE TABLE IF NOT EXISTS non_battle_consumable_item
(
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES item (id)
);

--- INVENTORY ---

CREATE TABLE IF NOT EXISTS player_inventory
(
    id        SERIAL PRIMARY KEY,
    player_id BIGINT  NOT NULL REFERENCES player (id),
    item_id   INT     NOT NULL REFERENCES item (id),
    amount    INT     NOT NULL DEFAULT 1,
    weight    REAL    NOT NULL DEFAULT 0,
    equipped  BOOLEAN NOT NULL DEFAULT FALSE
);

--- BOTS ---

CREATE TABLE IF NOT EXISTS bot
(
    id            SERIAL PRIMARY KEY,
    weapon_id     INT  NOT NULL REFERENCES weapon_item (id),
    name          TEXT NOT NULL,
    level         INT  NOT NULL DEFAULT 1,
    action_points INT  NOT NULL DEFAULT 6
);

--- MISC ---

CREATE FUNCTION read_only() RETURNS TRIGGER AS
$$
BEGIN
    RAISE EXCEPTION 'Cannot modify read-only table.';
END;
$$
    LANGUAGE plpgsql;

CREATE TRIGGER read_only_trigger
    BEFORE INSERT OR UPDATE OR DELETE
    ON player_class
EXECUTE PROCEDURE read_only();

--- BATTLE ---

CREATE TYPE WINNER AS ENUM ('en', 'fr', 'bots');

-- Maximum - 32 players/bots per battle are allowed. Battle record will be stored AFTER the battle is finished.
CREATE TABLE battle
(
    id         BIGSERIAL PRIMARY KEY,
    start_time TIMESTAMP         NOT NULL,
    end_time   TIMESTAMP         NOT NULL,
    winner     WINNER NOT NULL
);

CREATE TABLE battle_participant
(
    battle_id      BIGINT PRIMARY KEY NOT NULL REFERENCES battle (id),
    player_id      BIGINT REFERENCES player (id),
    bot_id         INT REFERENCES bot (id),
    outcome_damage INT                NOT NULL DEFAULT 0,
    income_damage  INT                NOT NULL DEFAULT 0,
    gained_exp     INT                NOT NULL DEFAULT 0,
    gained_valor   BOOLEAN            NOT NULL
);

CREATE TABLE battle_log
(
    id  BIGINT PRIMARY KEY REFERENCES battle (id),
    log TEXT NOT NULL
);

-- COMMIT TRANSACTION;