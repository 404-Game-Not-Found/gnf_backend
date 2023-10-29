use rusqlite::Connection;

use crate::utils::constants::db::{author, article, image, tags, article_tags, image_tags, DB_PATH};

use super::create_connection;


pub fn setup() -> Result<(), rusqlite::Error> {
    let conn = create_connection()?;
    
    create_author_table(&conn)?;
    
    create_article_table(&conn)?;
    
    create_image_table(&conn)?;
    
    create_tags_table(&conn)?;
    
    create_article_tags_table(&conn)?;
    
    create_image_tags_table(&conn)?;

    conn.execute("PRAGMA foreign_keys=ON;", [])?;

    let mut stmt = conn.prepare(
        "ALTER (?1) ADD FOREIGN KEY (?2) REFERENCES (?3)(id)"
    )?;

    stmt.execute([article::TABLE_NAME, author::TABLE_NAME, author::TABLE_NAME])?;
    
    stmt.execute([article_tags::TABLE_NAME, article::TABLE_NAME, article::TABLE_NAME])?;
    
    stmt.execute([article_tags::TABLE_NAME, tags::TABLE_NAME, tags::TABLE_NAME])?;
    
    stmt.execute([image_tags::TABLE_NAME, image::TABLE_NAME, image::TABLE_NAME])?;
    
    stmt.execute([image_tags::TABLE_NAME, tags::TABLE_NAME, tags::TABLE_NAME])?;

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
                {} {}
            );",
            article::TABLE_NAME,
            article::ID, "INTEGER", "PRIMARY", "KEY",
            article::TITLE, "TEXT",
            article::MARKDOWN_PATH, "TEXT",
            article::TAGS, "INTEGER",
            article::CREATED_AT, "DATETIME"
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
            image::IMAGE_PATH, "TEXT"
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
                {} {}
            );",
            article_tags::TABLE_NAME,
            article_tags::ID, "INTEGER", "PRIMARY", "KEY",
            article_tags::ARTICLE, "INTEGER",
            article_tags::TAG, "INTEGER"
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
                {} {}
            );",
            image_tags::TABLE_NAME,
            image_tags::ID, "INTEGER", "PRIMARY", "KEY",
            image_tags::IMAGE, "INTEGER",
            image_tags::TAG, "INTEGER"
        )
    )?;

    stmt.execute([])
}

