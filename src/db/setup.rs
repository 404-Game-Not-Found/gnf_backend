use rusqlite::Connection;

use crate::utils::constants::db::{author, article, image, tags, article_tags, image_tags, DB_PATH};

use super::create_connection;


pub fn setup() -> Result<(), rusqlite::Error> {
    let _ = std::fs::remove_file(DB_PATH);

    let conn = create_connection()?;
    
    create_author_table(&conn)?;
    
    create_article_table(&conn)?;
    
    create_image_table(&conn)?;
    
    create_tags_table(&conn)?;
    
    create_article_tags_table(&conn)?;
    
    create_image_tags_table(&conn)?;

    Ok(())
}


pub fn create_author_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut stmt = conn.prepare(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                {} {} {} {},
                {} {},
                {} {},
                {} {},
                {} {},
                {} {}
            );",
            author::TABLE_NAME,
            author::ID, "INTEGER", "PRIMARY", "KEY",
            author::FIRST_NAME, "TEXT",
            author::LAST_NAME, "TEXT",
            author::DISPLAY_NAME, "TEXT",
            author::EMAIL, "TEXT",
            author::PASSWORD, "TEXT"
        )
    )?;

    stmt.execute([])
}


pub fn create_article_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut stmt = conn.prepare(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                {} {} {} {}, 
                {} {},
                {} {},
                {} {},
                {} {},
                {} {},
                FOREIGN KEY ({}) REFERENCES {}({}),
                FOREIGN KEY ({}) REFERENCES {}({})
            );",
            article::TABLE_NAME,
            article::ID, "INTEGER", "PRIMARY", "KEY",
            article::TITLE, "TEXT",
            article::MARKDOWN_PATH, "TEXT",
            article::AUTHOR, "INTEGER",
            article::TAGS, "INTEGER",
            article::CREATED_AT, "DATETIME",
            article::TAGS, tags::TABLE_NAME, tags::ID,
            article::AUTHOR, author::TABLE_NAME, author::ID,
        )
    )?;

    stmt.execute([])
}



pub fn create_image_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut stmt = conn.prepare(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                {} {} {} {},
                {} {}
            );",
            image::TABLE_NAME,
            image::ID, "INTEGER", "PRIMARY", "KEY",
            image::IMAGE_PATH, "TEXT",

        )
    )?;

    stmt.execute([])
}


pub fn create_tags_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut stmt = conn.prepare(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                {} {} {} {},
                {} {}
            );",
            tags::TABLE_NAME,
            tags::ID, "INTEGER", "PRIMARY", "KEY",
            tags::TAG, "TEXT"
        )
    )?;

    stmt.execute([])
}



pub fn create_article_tags_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut stmt = conn.prepare(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                {} {} {} {},
                {} {},
                {} {},
                FOREIGN KEY ({}) REFERENCES {}({}),
                FOREIGN KEY ({}) REFERENCES {}({})
            );",
            article_tags::TABLE_NAME,
            article_tags::ID, "INTEGER", "PRIMARY", "KEY",
            article_tags::ARTICLE, "INTEGER",
            article_tags::TAG, "INTEGER",
            article_tags::ARTICLE, article::TABLE_NAME, article::ID,
            article_tags::TAG, image::TABLE_NAME, image::ID,
        )
    )?;

    stmt.execute([])
}



pub fn create_image_tags_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut stmt = conn.prepare(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                {} {} {} {},
                {} {},
                {} {},
                FOREIGN KEY ({}) REFERENCES {}({}),
                FOREIGN KEY ({}) REFERENCES {}({})
            );",
            image_tags::TABLE_NAME,
            image_tags::ID, "INTEGER", "PRIMARY", "KEY",
            image_tags::IMAGE, "INTEGER",
            image_tags::TAG, "INTEGER",
            image_tags::IMAGE, image::TABLE_NAME, image::ID,
            image_tags::TAG, tags::TABLE_NAME, tags::ID
        )
    )?;

    stmt.execute([])
}

