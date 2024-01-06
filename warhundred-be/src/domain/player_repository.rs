use diesel::prelude::*;

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

pub enum PlayerError {
    InternalServerError,
    NotFound(i32),
    CannotRegister(String),
}

impl IntoResponse for PostError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("PostModel with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"PostModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}

pub fn get_player(pool: &deadpool_diesel::postgres::Pool, nick: &str) -> QueryResult<Player> {
    use crate::schema::warhundred::player::dsl::*;

    player
        .filter(nickname.eq(nick))
        .first::<Player>(conn)
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

// TODO: Add error handling, rewrite on deadpool_diesel::postgres::Pool usage, make async
pub async fn register_player(
    pool: &deadpool_diesel::postgres::Pool,
    new_player: &InsertablePlayer,
) -> QueryResult<usize> {
    use crate::schema::warhundred::player::dsl::*;

    // TODO: Handle errors
    let conn = pool.get().await.unwrap();
    let res = conn
        .interact(|conn| {
            diesel::insert_into(player)
                .values(&new_player)
                .execute(conn)
        })
        .await;

    re
}