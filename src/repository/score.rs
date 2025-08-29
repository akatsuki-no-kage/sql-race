use chrono::NaiveDateTime;
use rusqlite::Connection;

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

pub struct ScoreRepository {
    connection: Connection,
}

impl ScoreRepository {
    pub fn new(database_file: &str) -> rusqlite::Result<Self> {
        let connection = rusqlite::Connection::open(database_file)?;

        Ok(Self { connection })
    }

    pub fn insert(&self, username: &str, score: u64) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO scores (username, score) VALUES (?, ?)",
            (username, score),
        )?;

        Ok(())
    }

    pub fn get_all(&self) -> rusqlite::Result<Vec<Score>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM scores ORDER BY score DESC")?;

        stmt.query_and_then((), |row| Score::try_from(row))?
            .collect()
    }
}
