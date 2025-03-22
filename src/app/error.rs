use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    //region entity handlers errors
    #[error("Cannot register new player with nickname {0}")]
    PlayerCannotRegister(String),
    #[error("Player with nickname {0} not found")]
    PlayerNotFound(String),
    //endregion
    
    //region database errors
    #[error("General database error happened during the query: {0}")]
    QueryError(String),
    #[error("Can't execute transaction: {0}")]
    TransactionError(String),
    //endregion
    
    #[error("Can't parse request body: {0}")]
    BodyParsingError(String)
}

// TODO: remove the detailed error explanation from IntoResponse. Just StatusCode is enough.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::PlayerCannotRegister(_) | Self::PlayerNotFound(_) => {
                (StatusCode::OK, self.to_string()).into_response()
            }
            Self::QueryError(e) | Self::TransactionError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::BodyParsingError(_) => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}
