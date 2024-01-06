use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,        // Represents an internal server error
    BodyParsingError(String),   // Represents an error related to request body parsing
}

// Define a util to create an internal server error
pub fn internal_error<E>(_err: E) -> AppError {
    AppError::InternalServerError
}

#[derive(Debug)]
pub enum InfraError {
    InternalServerError,
    // Represents an internal server error
    NotFound,            // Represents a resource not found error
}

pub fn adapt_infra_error<T: Error>(error: T) -> InfraError {
    error.as_infra_error()
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraError::NotFound => write!(f, "Not found"), // Display "Not found" for NotFound variant
            InfraError::InternalServerError => write!(f, "Internal server error"), // Display "Internal server error" for InternalServerError variant
        }
    }
}

#[derive(Debug)]
pub enum PlayerError {
    CannotRegister(String),
    NotFound(String),
    InfraError(InfraError),
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerError::CannotRegister(nick) => write!(f, "Cannot register new player with nickname {}", nick),
            PlayerError::NotFound(reason) => write!(f, "{}", reason),
            _ => write!(f, "Internal server error"),
        }
    }
}

// Define a custom Error trait for types that can be converted to InfraError
pub trait Error {
    fn as_infra_error(&self) -> InfraError;
}

impl Error for diesel::result::Error {
    fn as_infra_error(&self) -> InfraError {
        match self {
            diesel::result::Error::NotFound => InfraError::NotFound, // Map NotFound to InfraError::NotFound
            _ => InfraError::InternalServerError, // Map other errors to InfraError::InternalServerError
        }
    }
}

impl Error for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}

impl Error for deadpool_diesel::InteractError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}