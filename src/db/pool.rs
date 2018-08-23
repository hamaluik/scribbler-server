use std::ops::Deref;
use rocket::http::Status;
use rusqlite::Connection;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type DatabasePool = Pool<SqliteConnectionManager>;
pub struct DbConn(pub PooledConnection<SqliteConnectionManager>);

#[cfg(not(test))]
pub fn get_database_pool(db_file: &str) -> DatabasePool {
    let manager = SqliteConnectionManager::file(db_file);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("db pool") // TODO: better error handling
}

#[cfg(test)]
pub fn get_database_pool(db_file: &str) -> DatabasePool {
    let manager = SqliteConnectionManager::file(db_file);
    Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("db pool") // TODO: better error handling
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<DatabasePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &Connection.
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}