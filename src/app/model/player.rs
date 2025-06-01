use crate::model::DefaultModel;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use std::fmt::Debug;

#[derive(Selectable, Queryable, Insertable, Clone, Default)]
#[diesel(table_name = crate::schema::player)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    // TODO: i32 -> i64 should be changed in the future.
    //  The only reason it is done this way is that diesel maps int to i32.
    pub id: Option<i32>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub registration_time: Option<NaiveDateTime>,
    pub last_login_time: Option<NaiveDateTime>,
    pub guild_id: Option<i32>,
    pub banned: i32,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Default, Debug, Clone)]
#[diesel(table_name = crate::schema::player_attributes)]
pub struct PlayerAttributes {
    pub player_id: i32,
    pub class_id: i32,
    pub rank_id: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub physique: i32,
    pub luck: i32,
    pub intellect: i32,
    pub experience: i32,
    pub level: i32,
    pub valor: i32,
}

#[derive(Queryable, Insertable, Default, Debug, Clone)]
#[diesel(table_name = crate::schema::player_class_progress)]
pub struct PlayerClassProgress {
    pub id: Option<i32>,
    pub player_id: i32,
    pub class_id: i32,
    pub first_spec_progress: f32,
    pub second_spec_progress: f32,
    pub third_spec_progress: Option<f32>,
    pub total_progress: Option<f32>,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("nickname", &self.nickname)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .field("registration_time", &self.registration_time)
            .field("last_login_time", &self.last_login_time)
            .field("guild_id", &self.guild_id)
            .finish()
    }
}

impl DefaultModel for PlayerAttributes {
    fn default_model(id: i32) -> Self {
        Self {
            player_id: id,
            class_id: 0,
            rank_id: 1,
            strength: 3,
            dexterity: 3,
            physique: 3,
            luck: 0,
            intellect: 0,
            experience: 0,
            level: 1,
            valor: 0,
        }
    }
}
