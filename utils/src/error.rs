use std::result;
use std::backtrace::Backtrace;
use thiserror::Error;


/// A type alias that forces the usage of the custom error type.
pub type Result<T> = result::Result<T, GlobalError>;

/// Custom error type for handling errors.
#[derive(Error, Debug)]
pub enum GlobalError {
    #[error("Configuration Error")]
    ConfigError(#[from] config::ConfigError),
    #[error("Poison Error")]
    PoisonError,
    #[error("IO Error")]
    IoError {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[error("Clap Error")]
    ClapError(#[from] clap::Error),
    #[error("Undefined Error")]
    Undefined,
}

impl<T> From<std::sync::PoisonError<T>> for GlobalError {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        GlobalError::PoisonError
    }
}
