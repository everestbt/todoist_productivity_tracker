use jiff::{
    ToSpan, 
    Zoned, 
    civil::Date, 
};
use rusqlite::{
    params, 
    Connection,
}; 
use db_lib::db_manager;
use anyhow::Result;

use crate::{
    PARSER,
    PRINTER,
};

struct ExcludedWeek {
    id: i32,
    week_start: String,
}

pub fn get_excluded_weeks() -> Result<Vec<Date>> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = db_manager::get_connection();
    create_table(&conn)?;
    
    let mut stmt = conn.prepare("SELECT id, week_start FROM excluded_weeks")?;
    let day_iter = stmt.query_map([], |row| {
        Ok(ExcludedWeek {
            id: row.get(0)?,
            week_start: row.get(1)?
        })
    })?;

    let mut day_vec : Vec<Date> = Vec::new();
    for d in day_iter {
        let parse = PARSER.parse_date(&d?.week_start)?;
        day_vec.push(parse);
    }
    Ok(day_vec)
}

pub fn exclude_week(day: Date) -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = db_manager::get_connection();
    create_table(&conn)?;
    
    // First remove any unneeded weeks
    remove_old_weeks(&conn)?;

    // Add in the new excluded week
    conn.execute(
        "INSERT INTO excluded_weeks (week_start) VALUES (?1)",
        params![PRINTER.date_to_string(&day)],
    )?;

    Ok(())
}

// Any day older than 7 days can be safely deleted
fn remove_old_weeks(conn: &Connection) -> Result<()> {
    let today = Zoned::now().date();
    // Calculate the day 5 weeks back, simplest calculation and always correct
    let limit = today.checked_sub(5.weeks())?;

    let mut stmt = conn.prepare("SELECT id, week_start FROM excluded_weeks")?;
    let day_iter = stmt.query_map([], |row| {
        Ok(ExcludedWeek {
            id: row.get(0)?,
            week_start: row.get(1)?
        })
    })?;

    for d in day_iter {
        let val = d?;
        let parse = PARSER.parse_date(&val.week_start)?;
        if parse.lt(&limit) {
            conn.execute(
                "DELETE FROM excluded_weeks WHERE id = ?1",
                params![val.id],
            )?;
        }
    }

    Ok(())
}

pub fn purge() -> Result<()> {
    let conn: Connection = db_manager::get_connection();
    conn.execute(
        "DROP TABLE IF EXISTS excluded_weeks",
        [], // No parameters needed
    )?;

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS excluded_weeks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            week_start TEXT NOT NULL
        )",
        [], // No parameters needed
    )?;

    Ok(())
}