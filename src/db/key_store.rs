use rusqlite::{params, Connection, Result}; // For database operations and result handling

use todoist_productivity_tracker::db_lib::db_manager;

struct Key {
    key: String,
}

pub fn get_key() -> Result<String> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = db_manager::get_connection();
    create_table(&conn)?;
    
    let mut stmt = conn.prepare("SELECT key FROM todoist_key")?;
    let mut result = stmt.query_map([], |row| {
        Ok(Key {
            key: row.get(0)?
        })
    })?;

    let key: String = result.next()
        .expect("Failed to load the key, use --key at least once")
        .expect("Failed to load the key")
        .key;
    Ok(key)
}

pub fn save_key(key: &String) -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = db_manager::get_connection();
    create_table(&conn)?;
    
    // Clear the table
    conn.execute(
        "DELETE FROM todoist_key",
        [], // No parameters needed
    )?;

    // Add in the key
    conn.execute(
        "INSERT INTO todoist_key (key) VALUES (?1)",
        params![key],
    )?;

    Ok(())
}

pub fn purge() -> Result<()> {
    let conn: Connection = db_manager::get_connection();
    conn.execute(
        "DROP TABLE IF EXISTS todoist_key",
        [], // No parameters needed
    )?;

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todoist_key (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL
        )",
        [], // No parameters needed
    )?;

    Ok(())
}