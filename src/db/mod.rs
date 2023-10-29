use rusqlite::Connection;

use crate::utils::constants::db::DB_PATH;

pub mod entities;

pub mod setup;


pub fn create_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open(DB_PATH)
}