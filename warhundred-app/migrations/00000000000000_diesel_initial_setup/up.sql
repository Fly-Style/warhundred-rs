-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

-- BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE;

-- ATTACH DATABASE 'test.db' AS warhundred;

--- MAP OBJECTS ---

-- BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS map_location
(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "name"              TEXT                              NOT NULL,
    "location"          POINT                             NOT NULL,
    location_difficulty INTEGER                           NOT NULL DEFAULT 1,
    movement_accel      FLOAT                             NOT NULL DEFAULT 1,
    aggression_prob     FLOAT                             NOT NULL
);

CREATE TABLE IF NOT EXISTS town_location
(
    id     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "name" TEXT                              NOT NULL,
    UNIQUE ("name")
);

--- GUILDS ---

CREATE TABLE IF NOT EXISTS guild
(
    id     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "name" TEXT                              NOT NULL,
    rank   INTEGER                           NOT NULL DEFAULT 1,
    UNIQUE ("name")
);

--- PLAYERS ---

CREATE TABLE IF NOT EXISTS player
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    nickname           TEXT                              NOT NULL,
    email              TEXT                              NOT NULL,
    password           TEXT                              NOT NULL,
    last_login         TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_map_location  INTEGER                           NOT NULL,
    last_town_location INTEGER                           NOT NULL,
    guild_id           INTEGER REFERENCES guild (id),
--     last_map_location  INTEGER       NOT NULL REFERENCES map_location (id),
--     last_town_location INTEGER REFERENCES town_location (id),
    UNIQUE (nickname, email)
);

CREATE UNIQUE INDEX IF NOT EXISTS player_nickname ON player (nickname);

CREATE TABLE IF NOT EXISTS player_class
(
    class_id   INTEGER     NOT NULL PRIMARY KEY,
    class_name VARCHAR(16) NOT NULL
);

INSERT INTO player_class
VALUES (0, 'no-class'),
       (1, 'warrior'),
       (2, 'archer'),
       (3, 'healer'),
       (4, 'rogue'),
       (5, 'lancer');

CREATE TABLE IF NOT EXISTS player_attributes
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    class_id   INTEGER                           NOT NULL REFERENCES player_class (class_id),
    player_id  INTEGER                           NOT NULL REFERENCES player (id),
    strength   INTEGER                           NOT NULL DEFAULT 0,
    dexterity  INTEGER                           NOT NULL DEFAULT 0,
    physique   INTEGER                           NOT NULL DEFAULT 0,
    luck       INTEGER                           NOT NULL DEFAULT 0,
    intellect  INTEGER                           NOT NULL DEFAULT 0,
    experience INTEGER                           NOT NULL,
    level      INTEGER                           NOT NULL DEFAULT 0,
    valor      INTEGER                           NOT NULL DEFAULT 0,
    rank       VARCHAR(32)                       NOT NULL DEFAULT 0
);

--- ITEMS ---

CREATE TABLE IF NOT EXISTS item
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name          TEXT                              NOT NULL,
    level_req     INTEGER                           NOT NULL DEFAULT 0,
    strength_req  INTEGER                           NOT NULL DEFAULT 0,
    dexterity_req INTEGER                           NOT NULL DEFAULT 0,
    physique_req  INTEGER                           NOT NULL DEFAULT 0,
    intellect_req INTEGER                           NOT NULL DEFAULT 0,
    valor_req     INTEGER                           NOT NULL DEFAULT 0,
    class_req     INTEGER REFERENCES player_class (class_id)
);

CREATE TABLE IF NOT EXISTS weapon_item
(
    id                   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    item_id              INTEGER                           NOT NULL REFERENCES item (id),
    action_points_to_use INTEGER                           NOT NULL DEFAULT 2,
    basic_damage         INTEGER                           NOT NULL DEFAULT 0,
    "range"              INTEGER                           NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS gear_item
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    item_id INTEGER                           NOT NULL REFERENCES item (id)
);

CREATE TABLE IF NOT EXISTS battle_consumable_item
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    item_id INTEGER                           NOT NULL REFERENCES item (id)
);

CREATE TABLE IF NOT EXISTS non_battle_consumable_item
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    item_id INTEGER                           NOT NULL REFERENCES item (id)
);

--- INVENTORY ---

CREATE TABLE IF NOT EXISTS player_inventory
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    player_id INTEGER                           NOT NULL REFERENCES player (id),
    item_id   INTEGER                           NOT NULL REFERENCES item (id),
    amount    INTEGER                           NOT NULL DEFAULT 1,
    weight    REAL                              NOT NULL DEFAULT 0,
    equipped  BOOLEAN                           NOT NULL DEFAULT FALSE
);

--- BOTS ---

CREATE TABLE IF NOT EXISTS bot
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    weapon_id     INTEGER                           NOT NULL REFERENCES weapon_item (id),
    name          TEXT                              NOT NULL,
    level         INTEGER                           NOT NULL DEFAULT 1,
    action_points INTEGER                           NOT NULL DEFAULT 6
);

--- BATTLE ---

CREATE TABLE factions
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(3)          NOT NULL
);

INSERT INTO factions
VALUES (0, 'en'),
       (1, 'fr'),
       (2, 'bots');


-- Maximum - 32 players/bots per battle are allowed. Battle record will be stored AFTER the battle is finished.
CREATE TABLE battle
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    start_time TIMESTAMP                         NOT NULL,
    end_time   TIMESTAMP                         NOT NULL,
    winner     INTEGER REFERENCES factions (id)  NOT NULL
);

CREATE TABLE battle_participant
(
    battle_id      INTEGER PRIMARY KEY            NOT NULL REFERENCES battle (id),
    player_id      INTEGER REFERENCES player (id) NOT NULL,
    bot_id         INTEGER REFERENCES bot (id)    NOT NULL,
    outcome_damage INTEGER                        NOT NULL DEFAULT 0,
    income_damage  INTEGER                        NOT NULL DEFAULT 0,
    gained_exp     INTEGER                        NOT NULL DEFAULT 0,
    gained_valor   BOOLEAN                        NOT NULL
);

CREATE TABLE battle_log
(
    id  INTEGER PRIMARY KEY REFERENCES battle (id) NOT NULL,
    log TEXT                                       NOT NULL
);

-- COMMIT TRANSACTION;