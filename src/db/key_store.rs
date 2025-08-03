use rusqlite::{params, Connection, Result}; // For database operations and result handling

static DATABASE_NAME: &'static str = "todoist_productivity_tracker_database.db";

struct Key {
    key: String,
}

pub fn get_key() -> Result<String> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = Connection::open(DATABASE_NAME)?;
    create_table(&conn)?;
    
    let mut stmt = conn.prepare("SELECT key FROM todoist_key")?;
    let mut result = stmt.query_map([], |row| {
        Ok(Key {
            key: row.get(0)?
        })
    })?;

    let key: String = result.next()
        .expect("Failed to load")
        .expect("Found no key, use --key first")
        .key;
    Ok(key)
}

pub fn save_key(key: &String) -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = Connection::open(DATABASE_NAME)?;
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
    let conn: Connection = Connection::open(DATABASE_NAME)?;
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