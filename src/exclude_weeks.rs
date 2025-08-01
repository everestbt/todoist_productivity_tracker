use chrono::NaiveDate;
use rusqlite::{params, Connection, Result}; // For database operations and result handling

static DATABASE_NAME: &'static str = "todoist_productivity_tracker_database.db";

struct ExcludedWeek {
    id: i32,
    week_start: String,
}

pub fn get_excluded_weeks() -> Result<Vec<NaiveDate>> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = Connection::open(DATABASE_NAME)?;
    create_table(&conn)?;
    
    let mut stmt = conn.prepare("SELECT id, week_start FROM excluded_weeks")?;
    let day_iter = stmt.query_map([], |row| {
        Ok(ExcludedWeek {
            id: row.get(0)?,
            week_start: row.get(1)?
        })
    })?;

    let mut day_vec : Vec<NaiveDate> = Vec::new();
    for d in day_iter {
        let parse = NaiveDate::parse_from_str(&d.unwrap().week_start.to_owned(), "%Y-%m-%d");
        match parse.is_err() {
            true => panic!("{}",parse.unwrap_err().to_string()),
            false => day_vec.push(parse.unwrap()),
        }
    }
    Ok(day_vec)
}

pub fn exclude_week(day: NaiveDate) -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = Connection::open(DATABASE_NAME)?;
    create_table(&conn)?;
    
    conn.execute(
        "INSERT INTO excluded_weeks (week_start) VALUES (?1)",
        params![day.format("%Y-%m-%d").to_string()],
    )?;

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {

    // Create a table named users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS excluded_weeks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            week_start TEXT NOT NULL
        )",
        [], // No parameters needed
    )?;

    Ok(())
}