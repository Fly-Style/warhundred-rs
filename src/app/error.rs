use thiserror::Error;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,      // Represents an internal server error
    BodyParsingError(String), // Represents an error related to request body parsing
}

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Cannot register new player with nickname {0}")]
    CannotRegister(String),
    #[error("Player with nickname {0} not found")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("General database error happened during the query: {0}")]
    GeneralError(String),
    #[error("Can't execute transaction: {0}")]
    TransactionError(String),
}
