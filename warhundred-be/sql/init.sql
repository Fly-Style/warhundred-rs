--- Note: this file will contain all initial tables until first alpha.

CREATE SCHEMA IF NOT EXISTS warhundred;

--- MISC ---

CREATE FUNCTION read_only() RETURNS TRIGGER AS
    $$
BEGIN
    RAISE EXCEPTION 'Cannot modify read-only table.';
END;
$$
LANGUAGE plpgsql;

--- MAP OBJECTS ---

CREATE TABLE IF NOT EXISTS warhundred.map_location
(
    id                  SERIAL PRIMARY KEY,
    "name"              TEXT  NOT NULL,
    "location"          POINT NOT NULL,
    location_difficulty INT   NOT NULL DEFAULT 1,
    movement_accel      FLOAT NOT NULL DEFAULT 1,
    aggression_prob     FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS warhundred.town_location
(
    id     SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    UNIQUE ("name")
    );

--- GUILDS ---

CREATE TABLE IF NOT EXISTS warhundred.guild
(
    id     SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    rank   INT  NOT NULL DEFAULT 1,
    UNIQUE ("name")
    );

--- PLAYERS ---

CREATE TABLE IF NOT EXISTS warhundred.player
(
    id                 SERIAL PRIMARY KEY,
    nickname           TEXT      NOT NULL,
    email              TEXT      NOT NULL,
    password           TEXT      NOT NULL,
    last_login         TIMESTAMP NOT NULL DEFAULT NOW(),
    last_map_location  INT       NOT NULL REFERENCES warhundred.map_location (id),
    last_town_location INT REFERENCES warhundred.town_location (id),
    guild_id           INT REFERENCES warhundred.guild (id),
    UNIQUE (nickname, email)
    );

CREATE TABLE IF NOT EXISTS warhundred.player_class
(
    class_id   INT         NOT NULL PRIMARY KEY,
    class_name VARCHAR(16) NOT NULL
    );

INSERT INTO warhundred.player_class
VALUES
    (0, 'no-class'),
    (1, 'warrior'),
    (2, 'archer'),
    (3, 'healer'),
    (4, 'rogue'),
    (5, 'lancer');

CREATE TRIGGER read_only_trigger
    BEFORE INSERT OR UPDATE OR DELETE
                     ON warhundred.player_class
                         EXECUTE PROCEDURE read_only();

CREATE TABLE IF NOT EXISTS warhundred.player_attributes
(
    id         SERIAL PRIMARY KEY,
    class_id   INT         NOT NULL REFERENCES warhundred.player_class (class_id),
    player_id  INT         NOT NULL REFERENCES warhundred.player (id),
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

CREATE TABLE IF NOT EXISTS warhundred.item
(
    id            SERIAL PRIMARY KEY,
    name          TEXT NOT NULL,
    level_req     INT  NOT NULL DEFAULT 0,
    strength_req  INT  NOT NULL DEFAULT 0,
    dexterity_req INT  NOT NULL DEFAULT 0,
    physique_req  INT  NOT NULL DEFAULT 0,
    intellect_req INT  NOT NULL DEFAULT 0,
    valor_req     INT  NOT NULL DEFAULT 0,
    class_req     INT REFERENCES warhundred.player_class (class_id)
    );

CREATE TABLE IF NOT EXISTS warhundred.weapon_item
(
    id                   SERIAL PRIMARY KEY,
    item_id              INT NOT NULL REFERENCES warhundred.item (id),
    action_points_to_use INT NOT NULL DEFAULT 2,
    basic_damage         INT NOT NULL DEFAULT 0,
    "range"              INT NOT NULL DEFAULT 1
    );

CREATE TABLE IF NOT EXISTS warhundred.gear_item
(
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES warhundred.item (id)
    );

CREATE TABLE IF NOT EXISTS warhundred.battle_consumable_item
(
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES warhundred.item (id)
    );

CREATE TABLE IF NOT EXISTS warhundred.non_battle_consumable_item
(
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES warhundred.item (id)
    );

--- INVENTORY ---

CREATE TABLE IF NOT EXISTS warhundred.player_inventory
(
    id        SERIAL PRIMARY KEY,
    player_id INT     NOT NULL REFERENCES warhundred.player (id),
    item_id   INT     NOT NULL REFERENCES warhundred.item (id),
    amount    INT     NOT NULL DEFAULT 1,
    weight    REAL    NOT NULL DEFAULT 0,
    equipped  BOOLEAN NOT NULL DEFAULT FALSE
    );

--- BOTS ---

CREATE TABLE IF NOT EXISTS warhundred.bot
(
    id            SERIAL PRIMARY KEY,
    weapon_id     INT  NOT NULL REFERENCES warhundred.weapon_item (id),
    name          TEXT NOT NULL,
    level         INT  NOT NULL DEFAULT 1,
    action_points INT  NOT NULL DEFAULT 6
    );

--- BATTLE ---

CREATE TYPE warhundred.WINNER AS ENUM ('en', 'fr', 'bots');

-- Maximum - 32 players/bots per battle are allowed. Battle record will be stored AFTER the battle is finished.
CREATE TABLE warhundred.battle
(
    id         SERIAL PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time   TIMESTAMP NOT NULL,
    winner     warhundred.WINNER    NOT NULL
);

CREATE TABLE warhundred.battle_participant
(
    battle_id      INT     NOT NULL REFERENCES warhundred.battle (id),
    player_id      INT REFERENCES warhundred.player (id),
    bot_id         INT REFERENCES warhundred.bot (id),
    outcome_damage INT     NOT NULL DEFAULT 0,
    income_damage  INT     NOT NULL DEFAULT 0,
    gained_exp     INT     NOT NULL DEFAULT 0,
    gained_valor   BOOLEAN NOT NULL
);

CREATE TABLE warhundred.battle_log
(
    battle_id INT PRIMARY KEY NOT NULL REFERENCES warhundred.battle (id),
    log       TEXT NOT NULL
);