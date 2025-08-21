use anyhow::Result;
use futures::{stream::FuturesUnordered, TryStreamExt};
use sqlx::{Executor, Row, SqlitePool};

#[derive(Debug)]
pub struct Column {
    pub id: u32,
    pub name: String,
    pub primary_key: bool,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>, // Nullable in the schema
}

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Schema {
    async fn from_name(name: String, pool: &SqlitePool) -> Result<Schema> {
        let columns = sqlx::query(&format!("PRAGMA table_info({})", &name))
            .fetch_all(pool)
            .await?;

        let columns = columns
            .into_iter()
            .map(|raw| {
                Column {
                    id: raw.get::<i32, _>("cid") as u32,
                    name: raw.get("name"),
                    data_type: raw.get("type"),
                    is_nullable: raw.get::<i32, _>("notnull") != 0, // Convert 0/1 to bool
                    default_value: raw.get("dflt_value"),           // Option<String> to handle NULL
                    primary_key: raw.get::<i32, _>("pk") != 0,      // Convert 0/1 to bool
                }
            })
            .collect();

        Ok(Schema {
            name: name.clone(),
            columns,
        })
    }
}

#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub raw_schema: String,
    pub schemas: Vec<Schema>,
}

impl Question {
    pub async fn new(question: String, answer: String, raw_schema: String) -> Result<Self> {
        let pool = &SqlitePool::connect("sqlite::memory:").await?;
        pool.execute(raw_schema.as_str()).await?;

        let table_names: Vec<String> =
            sqlx::query!("SELECT name FROM sqlite_schema WHERE type = 'table'")
                .fetch_all(pool)
                .await?
                .into_iter()
                .map(|record| record.name)
                .try_collect()
                .unwrap();

        let schemas = table_names
            .into_iter()
            .map(|name| Schema::from_name(name, &pool))
            .collect::<FuturesUnordered<_>>()
            .try_collect()
            .await?;

        Ok(Question {
            question,
            answer,
            raw_schema,
            schemas,
        })
    }
}
