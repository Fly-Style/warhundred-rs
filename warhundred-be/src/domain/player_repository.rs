use diesel::prelude::*;
use crate::error::PlayerError;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::warhundred::player)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub last_login: std::time::SystemTime,
    pub last_map_location: i32,
    pub last_town_location: Option<i32>,
    pub guild_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::warhundred::player)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertablePlayer {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub last_login: std::time::SystemTime,
    pub last_map_location: i32,
    pub last_town_location: Option<i32>,
    pub guild_id: Option<i32>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::warhundred::player)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlayerView {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub last_login: std::time::SystemTime,
    pub last_map_location: i32,
    pub last_town_location: Option<i32>,
    pub guild_id: Option<i32>,
}

pub async fn get_player(pool: &deadpool_diesel::postgres::Pool, nick: String) -> Result<PlayerView, PlayerError> {
    use crate::schema::warhundred::player::dsl::*;
    let conn = pool.get().await.unwrap();
    let _nick = nick.clone();

    let res = conn
        .interact(move |conn| {
            player
                .filter(nickname.eq(nick))
                .first::<Player>(conn)
        })
        .await;
    // TODO: understand map_err
    // .map_err(adapt_infra_error)?
    // .map_err(adapt_infra_error)?;

    match res {
        Ok(qr) => {
            match qr {
                Ok(res) => {
                    return Ok(adapt_player_to_player_view(res));
                }
                Err(_) => Err(PlayerError::NotFound(_nick))
            }
        }
        Err(_) => Err(PlayerError::NotFound(_nick))
    }
}

fn adapt_player_to_player_view(new_player: Player) -> PlayerView {
    PlayerView {
        id: new_player.id,
        nickname: new_player.nickname,
        email: new_player.email,
        last_login: new_player.last_login,
        last_map_location: 0,
        last_town_location: None,
        guild_id: None,
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::warhundred::player)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SafePlayerView {
    pub id: i32,
    pub nickname: String,
}

pub async fn register_player(
    pool: &deadpool_diesel::postgres::Pool,
    new_player: InsertablePlayer,
) -> Result<SafePlayerView, PlayerError> {
    use crate::schema::warhundred::player::dsl::*;

    println!("Registering player: {:?}", new_player);

    let nick = new_player.nickname.clone();
    let conn = pool.get().await.unwrap();
    let res = conn
        .interact(move |conn| {
            diesel::insert_into(player)
                .values(new_player)
                .returning(Player::as_returning()) // Return the inserted playa
                .get_result(conn)
        })
        .await;

    match res {
        Ok(qr) => {
            match qr {
                Ok(res) => {
                    return Ok(SafePlayerView { id: res.id, nickname: res.nickname });
                }
                Err(_) => Err(PlayerError::CannotRegister(String::from(nick))),
            }
        }
        Err(_) => Err(PlayerError::CannotRegister(String::from(nick))),
    }
}