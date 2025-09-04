use std::fs;

use chrono::NaiveDateTime;
use rusqlite::{Connection, OptionalExtension};

use crate::config::CONFIG;

#[derive(Debug)]
pub struct Score {
    pub username: String,
    pub score: u64,
    pub created_at: NaiveDateTime,
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for Score {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            username: row.get("username")?,
            score: row.get("score")?,
            created_at: row.get("created_at")?,
        })
    }
}

const SCHEMA: &str = include_str!("../../schema.sql");

fn new_connection(database_file: &str) -> rusqlite::Result<Connection> {
    let existed = fs::exists(database_file).unwrap();

    let connection = rusqlite::Connection::open(database_file)?;
    if !existed {
        connection.execute_batch(SCHEMA)?;
    }

    Ok(connection)
}

pub fn is_user_exist(username: &str) -> rusqlite::Result<bool> {
    let connection = new_connection(&CONFIG.database_file)?;

    match connection
        .query_row(
            "SELECT id FROM scores WHERE username = ?",
            [username],
            |_| Ok(()),
        )
        .optional()
    {
        Ok(None) => Ok(true),
        Ok(Some(_)) | Err(_) => Ok(false),
    }
}

pub fn insert(username: &str, score: u64) -> rusqlite::Result<()> {
    let connection = new_connection(&CONFIG.database_file)?;

    connection.execute(
        "INSERT INTO scores (username, score) VALUES (?, ?)",
        (username, score),
    )?;

    Ok(())
}

pub fn get_all() -> rusqlite::Result<Vec<Score>> {
    let connection = new_connection(&CONFIG.database_file)?;

    let mut stmt = connection.prepare("SELECT * FROM scores ORDER BY score DESC")?;

    stmt.query_and_then((), |row| Score::try_from(row))?
        .collect()
}
