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
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    nickname          TEXT NOT NULL,
    email             TEXT NOT NULL,
    password          TEXT NOT NULL, -- hashed password
    registration_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login_time   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    guild_id          INTEGER REFERENCES guild (id),

--     last_map_location  INTEGER       NOT NULL REFERENCES map_location (id),
--     last_town_location INTEGER REFERENCES town_location (id),
    UNIQUE (nickname, email)
);

CREATE UNIQUE INDEX IF NOT EXISTS player_nickname ON player (nickname);
CREATE UNIQUE INDEX IF NOT EXISTS player_creds ON player (email, password);

CREATE TABLE IF NOT EXISTS player_attributes
(
    player_id  INTEGER NOT NULL PRIMARY KEY REFERENCES player (id),
    class_id   INTEGER NOT NULL REFERENCES player_class (class_id) DEFAULT 1,
    rank_id    INTEGER NOT NULL REFERENCES player_rank_table (id)  DEFAULT 1,
    strength   INTEGER NOT NULL                                    DEFAULT 0,
    dexterity  INTEGER NOT NULL                                    DEFAULT 0,
    physique   INTEGER NOT NULL                                    DEFAULT 0,
    luck       INTEGER NOT NULL                                    DEFAULT 0,
    intellect  INTEGER NOT NULL                                    DEFAULT 0,
    experience INTEGER NOT NULL,
    level      INTEGER NOT NULL                                    DEFAULT 0,
    valor      INTEGER NOT NULL                                    DEFAULT 0
);

CREATE TABLE player_class_progress
(
    player_id            INTEGER NOT NULL,
    class_id             INTEGER NOT NULL,
    first_spec_progress  REAL    NOT NULL DEFAULT 0,
    second_spec_progress REAL    NOT NULL DEFAULT 0,
    third_spec_progress  REAL             DEFAULT NULL,
    total_progress       REAL GENERATED ALWAYS AS ((first_spec_progress + second_spec_progress) / 2) STORED
);

--- PLAYER STATIC TABLES ---

CREATE TABLE IF NOT EXISTS player_class
(
    class_id             INTEGER    NOT NULL PRIMARY KEY,
    class_name           VARCHAR(8) NOT NULL,
    class_spec_one_name  VARCHAR(32),
    class_spec_two_name  VARCHAR(32),
    class_spec_tree_name VARCHAR(32)
);

INSERT INTO player_class
VALUES (0, 'no-class', NULL, NULL, NULL),
       (1, 'Warrior', 'One-handed weapons', 'Two-handed weapons', 'Shield'),
       (2, 'Archer', 'Bow', 'Crossbow', 'Dexterity'),
       (3, 'Healer', 'Therapy', 'Surgery', 'Buffing'),
       (4, 'Rogue', 'Stealth', 'Traps', NULL),
       (5, 'Lancer', 'Horseriding', 'Crushing Weapons', 'Impaling Weapons');

CREATE TABLE IF NOT EXISTS player_experience_table
(
    exp   INTEGER NOT NULL PRIMARY KEY, -- experience for the next up
    up    INTEGER NOT NULL,             -- 'up' number within the level
    level INTEGER NOT NULL,             -- level
    attrs INTEGER NOT NULL,             -- attributes point granted by reaching the up
    money INTEGER NOT NULL              -- attributes point granted by reaching the up
);

-- TODO: fetch into some cache during the server start
INSERT INTO player_experience_table
VALUES (0, 0, 0, 3, 50),
       (25, 0, 1, 3, 100),
       (100, 1, 1, 1, 100),
       (250, 0, 2, 4, 200),
       (400, 1, 2, 1, 200),
       (650, 2, 2, 1, 250),
       (1000, 0, 3, 4, 400),
       (1500, 1, 3, 1, 250),
       (2500, 2, 3, 1, 300),
       (4500, 0, 4, 4, 500),
       (1000000, 0, 5, 4, 2500);

CREATE TABLE IF NOT EXISTS player_rank_table
(
    id              INTEGER PRIMARY KEY NOT NULL,
    valor           INTEGER             NOT NULL, -- experience for the next up
    min_level       INTEGER             NOT NULL, -- level
    rank_name_EN    TEXT                NOT NULL,
    rank_name_FR    TEXT                NOT NULL,
    rank_name_UA    TEXT                NOT NULL,
    rank_pic_url_EN TEXT DEFAULT NULL,
    rank_pic_url_FR TEXT DEFAULT NULL
);

INSERT INTO player_rank_table (id, valor, min_level, rank_name_EN, rank_name_FR, rank_name_UA)
VALUES (1, 0, 0, 'Recruit', 'Recruter', 'Рекрут'),
       (2, 5, 2, 'Rookie', 'Débutant', 'Новобранец'),
       (3, 10, 3, 'Soldier', 'Soldat', 'Солдат'),
       (4, 25, 4, 'Senior Soldier', 'Soldat supérieur', 'Старший солдат'),
       (5, 50, 4, 'Foreman', 'Contremaître', 'Десятник'),
       (6, 75, 5, 'Sergeant', 'Sergeant', 'Сержант'),
       (7, 100, 5, 'Senior Sergeant', 'Sergent supérieur', 'Старший Сержант'),
       (8, 150, 6, 'Master Sergeant', 'Sergent-chef', 'Мастер-Сержант'),
       (9, 200, 6, 'First Sergeant', 'Premier Sergent', 'Первый Сержант'),
       (10, 300, 7, 'Cornet', 'Cornet', 'Корнет'),
       (11, 400, 7, 'Knight', 'Chevalier', 'Рыцарь'),
       (20, 10000, 10, 'Marshall', 'Marshall', 'Маршал');

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