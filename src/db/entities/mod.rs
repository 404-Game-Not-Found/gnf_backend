
use rusqlite::{Connection, Error};

use crate::utils::constants::db::DB_PATH;

pub mod author;
pub mod article;
pub mod article_image;

pub fn establish_connection() -> Result<Connection, Error> {
    Connection::open(DB_PATH)
}
