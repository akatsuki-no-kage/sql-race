use std::{env, ops::Deref};

pub struct Connection(rusqlite::Connection);

impl Default for Connection {
    fn default() -> Self {
        let database_file = env::var("DATABASE_FILE").unwrap_or("score.db".to_string());

        Self(rusqlite::Connection::open(database_file).unwrap())
    }
}

impl Deref for Connection {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
