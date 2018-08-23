use rusqlite;
use harsh;
use log;
use std::io;

#[derive(Debug)]
pub enum Error {
    DatabaseError(rusqlite::Error),
    PoolError,
    ConfigError(io::Error),
    HashIDError(harsh::Error),
    SetLoggerError(log::SetLoggerError),
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<harsh::Error> for Error {
    fn from(error: harsh::Error) -> Self {
        Error::HashIDError(error)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(error: log::SetLoggerError) -> Self {
        Error::SetLoggerError(error)
    }
}
