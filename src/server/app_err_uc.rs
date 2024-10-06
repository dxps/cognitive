//! This module provides:
//! - `AppError` - to abstract any infrastructure and low-level errors (like database related ones)
//!                and convert them into an app (domain) specific ones.
//! - `AppUseCase`s - relevant for the proper conversion from a low-level error to a higher (`AppError`) one.

use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    //
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error("")]
    Ignorable,

    #[error("internal error")]
    InternalErr,

    #[error("{0}")]
    Err(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::Err(s.to_string())
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::from(err.to_string())
    }
}

#[derive(Debug)]
pub enum AppUseCase {
    UserRegistration,
    UserLogin,
}
