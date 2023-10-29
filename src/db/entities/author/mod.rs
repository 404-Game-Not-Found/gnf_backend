use rusqlite::{Connection, Statement};
use serde::{Deserialize, Serialize};

use crate::utils::constants::db::author;



#[derive(Deserialize, Serialize)]
pub struct Author {
    pub username: String
}

impl Default for Author {
    fn default() -> Self {
        Self { username: "DEFAULT".to_string() }
    }
}


pub fn get_all_authors(conn: &Connection) -> Result<Vec<Author>, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("SELECT username FROM {}", author::TABLE_NAME))?;

    let author_iter = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?;

    let mut auths = Vec::new();

    for username in author_iter {
        match username {
            Ok(nm) =>
                auths.push(Author {
                    username: nm
                }),
            Err(_) => continue, // TODO: Keep track of errors
        }
        
    }

    Ok(auths)
}



pub fn get_authors_by_username(conn: &Connection, username: String) -> Result<Vec<Author>, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("SELECT username FROM {} WHERE username LIKE ?1", author::TABLE_NAME))?;

    // Places the username, where (?1) is, sanitizing the username
    let author_iter = stmt.query_map([&username], |row| {
        Ok(row.get(0)?)
    })?;

    let mut auths = Vec::new();

    for username in author_iter {
        match username {
            Ok(nm) =>
                auths.push(Author {
                    username: nm
                }),
            Err(_) => continue, // TODO: Keep track of errors
        }
        
    }

    Ok(auths)
}