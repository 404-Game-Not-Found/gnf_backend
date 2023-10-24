use std::fmt::format;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::utils::constants::db::{image::{SOURCE_FIELD, TAGS_FIELD, self}, DB_SEPERATOR};

#[derive(Deserialize, Serialize)]
pub struct ArticleImage {
    source: String,
    tags: Vec<String>
}

pub fn get_images_by_id(conn: &Connection, id: String) -> Result<Vec<ArticleImage>, rusqlite::Error> {
    Ok(
        conn
            .prepare(
                &format!("SELECT {}, {} FROM {} WHERE id LIKE {}", SOURCE_FIELD, TAGS_FIELD, image::TABLE_NAME, id)
            )?
            .query_map([], |row| {
                Ok((row.get::<usize, String>(0)?, row.get::<usize, String>(1)?))
            })?
            .into_iter()
            .filter_map(Result::ok)
            .into_iter()
            .map(
                |(src, tg)|
                    ArticleImage {
                        source: src,
                        tags: tg
                            .split(DB_SEPERATOR)
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>() }
            )
            .collect::<Vec<ArticleImage>>()
    )
}

pub fn get_images_by_tags(conn: &Connection, tags: Vec<String>) -> Result<Vec<ArticleImage>, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("SELECT {}, {} FROM {}", SOURCE_FIELD, TAGS_FIELD, image::TABLE_NAME))?;

    let image_iter = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get::<usize, String>(1)?))
    })?;

    let mut images = Vec::new();

    for el in image_iter {
        match el {
            Ok((src, tg)) => {

                let img = ArticleImage {
                    source: src,
                    tags: tg.split(DB_SEPERATOR).map(|s| s.to_string()).collect::<Vec<String>>(),
                };

                for t in tags.clone() {
                    if img.tags.contains(&t) {
                        images.push(img);
                        break;
                    }
                }

            }
            Err(_) => continue,
        }
    }

    Ok(images)
}