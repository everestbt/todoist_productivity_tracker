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

struct ExcludedDay {
    id: i32,
    day: String,
}

pub fn get_excluded_days() -> Result<Vec<Date>> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = db_manager::get_connection();
    create_table(&conn)?;
    
    let mut stmt = conn.prepare("SELECT id, day FROM excluded_days")?;
    let day_iter = stmt.query_map([], |row| {
        Ok(ExcludedDay {
            id: row.get(0)?,
            day: row.get(1)?
        })
    })?;

    let mut day_vec : Vec<Date> = Vec::new();
    for d in day_iter {
        let parse = PARSER.parse_date(&d?.day.to_owned())?;
        day_vec.push(parse);
    }
    Ok(day_vec)
}

pub fn exclude_day(day: Date) -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn: Connection = db_manager::get_connection();
    create_table(&conn)?;

    // First remove any unneeded days to keep it small
    remove_old_days(&conn)?;
    
    // Add in the new day
    conn.execute(
        "INSERT INTO excluded_days (day) VALUES (?1)",
        params![PRINTER.date_to_string(&day)],
    )?;

    Ok(())
}

// Any day older than 7 days can be safely deleted
fn remove_old_days(conn: &Connection) -> Result<()> {
    let today = Zoned::now().date();
    let limit = today.checked_sub(7.days())?;

    let mut stmt = conn.prepare("SELECT id, day FROM excluded_days")?;
    let day_iter = stmt.query_map([], |row| {
        Ok(ExcludedDay {
            id: row.get(0)?,
            day: row.get(1)?
        })
    })?;

    for d in day_iter {
        let val = d?;
        let parse = PARSER.parse_date(&val.day.to_owned())?;
        if parse.lt(&limit) {
            conn.execute(
                "DELETE FROM excluded_days WHERE id = ?1",
                params![val.id],
            )?;
        }
    }

    Ok(())
}

pub fn purge() -> Result<()> {
    let conn: Connection = db_manager::get_connection();
    conn.execute(
        "DROP TABLE IF EXISTS excluded_days",
        [], // No parameters needed
    )?;

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS excluded_days (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            day TEXT NOT NULL
        )",
        [], // No parameters needed
    )?;

    Ok(())
}