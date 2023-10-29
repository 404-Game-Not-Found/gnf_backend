use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

use crate::utils::constants::db::article;

use super::{author::Author, article_image::ArticleImage};


#[derive(Deserialize, Serialize)]
pub struct Article {
    pub author: Author,
    pub content: String,
    pub images: Vec<ArticleImage>,
    pub tags: Vec<String>
}

impl Default for Article {
    fn default() -> Self {
        Self { author: Default::default(), content: "DEFAULT CONTENT".to_string(), images: Default::default(), tags: Default::default() }
    }
}


pub fn get_all_articles(conn: &Connection) -> Result<Vec<Article>, Error> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", article::TABLE_NAME))?;

    let article_iter = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?;

    let mut arts = Vec::new();

    for partial_art in article_iter {
        arts.push(Article {
            content: partial_art?,
            ..Default::default()
        });
    }

    Ok(arts)
}