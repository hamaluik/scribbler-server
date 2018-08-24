use rusqlite::{Connection, Error};
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Source {
    File(PathBuf),
    _Memory,
}

/// An `r2d2::ManageConnection` for `rusqlite::Connection`s.
#[derive(Debug)]
pub struct SqliteConnectionManager(Source);

impl SqliteConnectionManager {
    /// Creates a new `SqliteConnectionManager` from file.
    ///
    /// See `rusqlite::Connection::open`
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        SqliteConnectionManager(Source::File(path.as_ref().to_path_buf()))
    }

    /// Creates a new `SqliteConnectionManager` from memory.
    pub fn _memory() -> Self {
        SqliteConnectionManager(Source::_Memory)
    }
}

impl r2d2::ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<Connection, Error> {
        match self.0 {
            Source::File(ref path) => Connection::open(path),
            Source::_Memory => Connection::open_in_memory(),
        }.map_err(Into::into)
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), Error> {
        conn.execute_batch("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}
