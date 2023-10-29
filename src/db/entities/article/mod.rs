use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

use crate::utils::constants::db::article;

use super::{author::Author, article_image::ArticleImage};


#[derive(Deserialize, Serialize)]
pub struct Article {
    id: usize,
    pub title: String,
    pub markdown_path: String,
    pub author: Author,
    pub created_at: String,
    pub tags: Vec<String>
}

impl Default for Article {
    fn default() -> Self {
        Self { 
            id: 0,
            author: Default::default(),
            tags: Default::default(),
            title: "DEFAULT TITLE".to_string(),
            markdown_path: "DEFAULT PATH".to_string(),
            created_at: "00.00.00".to_string(),
        }
    }
}


pub fn get_all_articles(conn: &Connection) -> Result<Vec<Article>, Error> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", article::TABLE_NAME))?;

    let article_iter = stmt.query_map([], |row| {
        Ok(row.get::<usize, String>(0)?)
    })?;

    let mut arts = Vec::new();

    for partial_art in article_iter {
        arts.push(Article::default());
    }

    Ok(arts)
}