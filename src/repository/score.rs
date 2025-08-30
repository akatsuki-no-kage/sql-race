use chrono::NaiveDateTime;
use rusqlite::Connection;

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
pub fn insert(username: &str, score: u64, database_file: &str) -> rusqlite::Result<()> {
    let connection = rusqlite::Connection::open(database_file)?;

    connection.execute(
        "INSERT INTO scores (username, score) VALUES (?, ?)",
        (username, score),
    )?;

    Ok(())
}

pub fn get_all(database_file: &str) -> rusqlite::Result<Vec<Score>> {
    let connection = rusqlite::Connection::open(database_file)?;

    let mut stmt = connection.prepare("SELECT * FROM scores ORDER BY score DESC")?;

    stmt.query_and_then((), |row| Score::try_from(row))?
        .collect()
}
