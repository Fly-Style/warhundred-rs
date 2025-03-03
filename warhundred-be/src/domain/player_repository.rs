use crate::error::{DatabaseError, PlayerError};
use crate::schema::player::dsl::*;
use crate::schema::player_attributes::dsl::player_attributes;
use axum_login::AuthUser;
use deadpool_diesel::sqlite::Pool;
use diesel::prelude::*;
use std::fmt::Debug;

type PlayerWithAttributes = (Player, PlayerAttributes);

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::player)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    // TODO: i32 -> i64 should be changed in the future.
    //  The only reason it is done this way is that diesel maps int to i32.
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub last_login: String,
    pub last_map_location: i32,
    pub last_town_location: i32,
    pub guild_id: Option<i32>,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("nickname", &self.nickname)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .field("last_login", &self.last_login)
            .field("last_map_location", &self.last_map_location)
            .field("last_town_location", &self.last_town_location)
            .field("guild_id", &self.guild_id)
            .finish()
    }
}

impl AuthUser for Player {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.nickname.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.nickname.as_bytes()
    }
}

// TODO: write integration tests for these functions
impl Player {
    pub async fn register_player(
        pool: &Pool,
        new_player: InsertablePlayer,
    ) -> Result<Player, PlayerError> {
        use crate::schema::player::dsl::*;

        println!("Registering player: {:?}", new_player);

        let nick = new_player.nickname.clone();
        let conn = pool.get().await.unwrap();
        let res = conn
            .interact(move |conn| {
                diesel::insert_into(player)
                    .values(new_player)
                    .returning(Player::as_returning())
                    .get_result(conn)
            })
            .await;

        match res {
            Ok(qr) => match qr {
                Ok(res) => Ok(res),
                Err(_) => Err(PlayerError::CannotRegister(nick)),
            },
            Err(_) => Err(PlayerError::CannotRegister(nick)),
        }
    }

    pub async fn get_player_by_nick(pool: &Pool, nick: String) -> Result<Player, PlayerError> {
        let conn = pool.get().await.unwrap();
        let _nick: String = nick.clone();

        let res = conn
            .interact(move |conn| player.filter(nickname.eq(nick)).first::<Player>(conn))
            .await;

        match res {
            Ok(qr) => match qr {
                Ok(res) => Ok(res),
                Err(_) => Err(PlayerError::NotFound(_nick)),
            },
            Err(_) => Err(PlayerError::NotFound(_nick)),
        }
    }
    
    pub async fn get_full_player_info_by_nick(pool: &Pool, nick: String) -> Result<PlayerWithAttributes, PlayerError> {
        let conn = pool.get().await.unwrap();
        let _nick: String = nick.clone();
        
        let res = conn
            .interact(move |conn| {
                player.filter(nickname.eq(nick))
                    .inner_join(player_attributes)
                    .first::<(Player, PlayerAttributes)>(conn)
            })
            .await;

        match res {
            Ok(qr) => match qr {
                Ok(res) => Ok(res),
                Err(_) => Err(PlayerError::NotFound(_nick)),
            },
            Err(_) => Err(PlayerError::NotFound(_nick)),
        }
    }

    pub async fn inc_valor(pool: &Pool, p_id: i32, rank_up: bool) -> Result<(), DatabaseError> {
        use crate::schema::player_attributes::dsl::*;
        let conn = pool.get().await.unwrap();
        let tx_res = conn.interact(move |conn| {
            conn.transaction(|conn| {
                diesel::update(player_attributes)
                    .filter(player_id.eq(p_id))
                    .set(valor.eq(valor + 1))
                    .execute(conn)?;
                if rank_up {
                    diesel::update(player_attributes)
                        .filter(player_id.eq(p_id))
                        .set(rank_id.eq(rank_id + 1))
                        .execute(conn)?;
                }
                Ok::<(), deadpool_diesel::Error>(())
            })
        })
        .await;

        match tx_res {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::TransactionError(e.to_string()))
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::player)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertablePlayer {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub last_login: String,
    pub last_map_location: i32,
    pub last_town_location: i32,
    pub guild_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Default, Debug, Clone)]
#[diesel(table_name = crate::schema::player_attributes)]
pub struct PlayerAttributes {
    // pub id: i32,
    pub class_id: i32,
    pub player_id: i32,
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
