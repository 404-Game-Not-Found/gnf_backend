use rusqlite::Connection;


pub fn setup() {

}


pub fn create_user_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", 1))?;

    Ok(())
}