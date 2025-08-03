use rusqlite::{params, Connection, Result}; // For database operations and result handling
use directories::{ProjectDirs};
use std::fs;

static DATABASE_NAME: &'static str = "todoist_productivity_tracker_database.db";

struct Key {
    key: String,
}

pub fn get_key() -> Result<String> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = get_connection();
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
    let conn: Connection = get_connection();
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
    let conn: Connection = get_connection();
    conn.execute(
        "DROP TABLE IF EXISTS todoist_key",
        [], // No parameters needed
    )?;

    Ok(())
}

fn get_connection() -> Connection {
    let binding = ProjectDirs::from("com", "everest", "todoist_productivity_tracker")
        .expect("Failed to get project directories");
    let data_dir =  binding.data_local_dir();
    if !fs::exists(data_dir).expect("Failed to check for directory") {
        fs::create_dir(data_dir).expect("Failed to create directory");
    }
    let path = data_dir.join(DATABASE_NAME);
    let conn: Connection = Connection::open(path).expect("Failed to open a connection");
    return conn;
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