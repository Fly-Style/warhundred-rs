// @generated automatically by Diesel CLI.

diesel::table! {
    battle (id) {
        id -> Integer,
        start_time -> Timestamp,
        end_time -> Timestamp,
        winner -> Integer,
    }
}

diesel::table! {
    battle_consumable_item (id) {
        id -> Integer,
        item_id -> Integer,
    }
}

diesel::table! {
    battle_log (id) {
        id -> Integer,
        log -> Text,
    }
}

diesel::table! {
    battle_participant (battle_id) {
        battle_id -> Integer,
        player_id -> Integer,
        bot_id -> Integer,
        outcome_damage -> Integer,
        income_damage -> Integer,
        gained_exp -> Integer,
        gained_valor -> Bool,
    }
}

diesel::table! {
    bot (id) {
        id -> Integer,
        weapon_id -> Integer,
        name -> Text,
        level -> Integer,
        action_points -> Integer,
    }
}

diesel::table! {
    factions (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    gear_item (id) {
        id -> Integer,
        item_id -> Integer,
    }
}

diesel::table! {
    guild (id) {
        id -> Integer,
        name -> Text,
        rank -> Integer,
    }
}

diesel::table! {
    item (id) {
        id -> Integer,
        name -> Text,
        level_req -> Integer,
        strength_req -> Integer,
        dexterity_req -> Integer,
        physique_req -> Integer,
        intellect_req -> Integer,
        valor_req -> Integer,
        class_req -> Nullable<Integer>,
    }
}

diesel::table! {
    map_location (id) {
        id -> Integer,
        name -> Text,
        location -> Integer,
        location_difficulty -> Integer,
        movement_accel -> Float,
        aggression_prob -> Float,
    }
}

diesel::table! {
    non_battle_consumable_item (id) {
        id -> Integer,
        item_id -> Integer,
    }
}

diesel::table! {
    player (id) {
        id -> Integer,
        nickname -> Text,
        email -> Text,
        password -> Text,
        last_login -> Timestamp,
        last_map_location -> Integer,
        last_town_location -> Integer,
        guild_id -> Nullable<Integer>,
    }
}

diesel::table! {
    player_attributes (id) {
        id -> Integer,
        class_id -> Integer,
        player_id -> Integer,
        strength -> Integer,
        dexterity -> Integer,
        physique -> Integer,
        luck -> Integer,
        intellect -> Integer,
        experience -> Integer,
        level -> Integer,
        valor -> Integer,
        rank -> Text,
    }
}

diesel::table! {
    player_class (class_id) {
        class_id -> Integer,
        class_name -> Text,
        class_spec_one_name -> Nullable<Text>,
        class_spec_two_name -> Nullable<Text>,
        class_spec_tree_name -> Nullable<Text>,
    }
}

diesel::table! {
    player_experience_table (exp) {
        exp -> Integer,
        up -> Integer,
        level -> Integer,
        attrs -> Integer,
        money -> Integer,
    }
}

diesel::table! {
    player_inventory (id) {
        id -> Integer,
        player_id -> Integer,
        item_id -> Integer,
        amount -> Integer,
        weight -> Float,
        equipped -> Bool,
    }
}

diesel::table! {
    player_rank_table (id) {
        id -> Integer,
        valor -> Integer,
        min_level -> Integer,
        rank_name_EN -> Text,
        rank_name_FR -> Text,
        rank_name_UA -> Text,
        rank_pic_url_EN -> Nullable<Text>,
        rank_pic_url_FR -> Nullable<Text>,
    }
}

diesel::table! {
    town_location (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    weapon_item (id) {
        id -> Integer,
        item_id -> Integer,
        action_points_to_use -> Integer,
        basic_damage -> Integer,
        range -> Integer,
    }
}

diesel::joinable!(battle -> factions (winner));
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
    factions,
    gear_item,
    guild,
    item,
    map_location,
    non_battle_consumable_item,
    player,
    player_attributes,
    player_class,
    player_experience_table,
    player_inventory,
    player_rank_table,
    town_location,
    weapon_item,
);
