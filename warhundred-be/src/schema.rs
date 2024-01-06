// @generated automatically by Diesel CLI.

pub mod warhundred {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "point", schema = "pg_catalog"))]
        pub struct Point;

        #[derive(diesel::query_builder::QueryId, std::fmt::Debug, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "winner", schema = "warhundred"))]
        pub struct Winner;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::Winner;

        warhundred.battle (id) {
            id -> Int4,
            start_time -> Timestamp,
            end_time -> Timestamp,
            winner -> Winner,
        }
    }

    diesel::table! {
        warhundred.battle_consumable_item (id) {
            id -> Int4,
            item_id -> Int4,
        }
    }

    diesel::table! {
        warhundred.battle_log (id) {
            id -> Int4,
            log -> Text,
        }
    }

    diesel::table! {
        warhundred.battle_participant (battle_id) {
            battle_id -> Int4,
            player_id -> Nullable<Int4>,
            bot_id -> Nullable<Int4>,
            outcome_damage -> Int4,
            income_damage -> Int4,
            gained_exp -> Int4,
            gained_valor -> Bool,
        }
    }

    diesel::table! {
        warhundred.bot (id) {
            id -> Int4,
            weapon_id -> Int4,
            name -> Text,
            level -> Int4,
            action_points -> Int4,
        }
    }

    diesel::table! {
        warhundred.gear_item (id) {
            id -> Int4,
            item_id -> Int4,
        }
    }

    diesel::table! {
        warhundred.guild (id) {
            id -> Int4,
            name -> Text,
            rank -> Int4,
        }
    }

    diesel::table! {
        warhundred.item (id) {
            id -> Int4,
            name -> Text,
            level_req -> Int4,
            strength_req -> Int4,
            dexterity_req -> Int4,
            physique_req -> Int4,
            intellect_req -> Int4,
            valor_req -> Int4,
            class_req -> Nullable<Int4>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::Point;

        warhundred.map_location (id) {
            id -> Int4,
            name -> Text,
            location -> Point,
            location_difficulty -> Int4,
            movement_accel -> Float8,
            aggression_prob -> Float8,
        }
    }

    diesel::table! {
        warhundred.non_battle_consumable_item (id) {
            id -> Int4,
            item_id -> Int4,
        }
    }

    diesel::table! {
        warhundred.player (id) {
            id -> Int4,
            nickname -> Text,
            email -> Text,
            password -> Text,
            last_login -> Timestamp,
            last_map_location -> Int4,
            last_town_location -> Nullable<Int4>,
            guild_id -> Nullable<Int4>,
        }
    }

    diesel::table! {
        warhundred.player_attributes (id) {
            id -> Int4,
            class_id -> Int4,
            player_id -> Int4,
            strength -> Int4,
            dexterity -> Int4,
            physique -> Int4,
            luck -> Int4,
            intellect -> Int4,
            experience -> Int8,
            level -> Int4,
            valor -> Int4,
            #[max_length = 32]
            rank -> Varchar,
        }
    }

    diesel::table! {
        warhundred.player_class (class_id) {
            class_id -> Int4,
            #[max_length = 16]
            class_name -> Varchar,
        }
    }

    diesel::table! {
        warhundred.player_inventory (id) {
            id -> Int4,
            player_id -> Int4,
            item_id -> Int4,
            amount -> Int4,
            weight -> Float4,
            equipped -> Bool,
        }
    }

    diesel::table! {
        warhundred.town_location (id) {
            id -> Int4,
            name -> Text,
        }
    }

    diesel::table! {
        warhundred.weapon_item (id) {
            id -> Int4,
            item_id -> Int4,
            action_points_to_use -> Int4,
            basic_damage -> Int4,
            range -> Int4,
        }
    }

    diesel::joinable!(battle_consumable_item -> item (item_id));
    diesel::joinable!(battle_log -> battle (id));
    diesel::joinable!(battle_participant -> battle (battle_id));
    diesel::joinable!(battle_participant -> bot (bot_id));
    diesel::joinable!(battle_participant -> player (player_id));
    diesel::joinable!(bot -> weapon_item (weapon_id));
    diesel::joinable!(gear_item -> item (item_id));
    diesel::joinable!(item -> player_class (class_req));
    diesel::joinable!(non_battle_consumable_item -> item (item_id));
    diesel::joinable!(player -> guild (guild_id));
    diesel::joinable!(player -> map_location (last_map_location));
    diesel::joinable!(player -> town_location (last_town_location));
    diesel::joinable!(player_attributes -> player (player_id));
    diesel::joinable!(player_attributes -> player_class (class_id));
    diesel::joinable!(player_inventory -> item (item_id));
    diesel::joinable!(player_inventory -> player (player_id));
    diesel::joinable!(weapon_item -> item (item_id));

    diesel::allow_tables_to_appear_in_same_query!(
        battle,
        battle_consumable_item,
        battle_log,
        battle_participant,
        bot,
        gear_item,
        guild,
        item,
        map_location,
        non_battle_consumable_item,
        player,
        player_attributes,
        player_class,
        player_inventory,
        town_location,
        weapon_item,
    );
}
