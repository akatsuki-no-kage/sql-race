use futures::TryStreamExt;
use sqlx::{Executor, SqlitePool};

use crate::TableInfo;

pub struct Schema {
    pub raw: String,
    pub table_infos: Vec<TableInfo>,
}

impl Schema {
    pub async fn new(raw: String) -> sqlx::Result<Self> {
        let pool = &SqlitePool::connect("sqlite::memory:").await?;
        pool.execute(raw.as_str()).await?;

        let table_names =
            sqlx::query_scalar::<_, String>("SELECT name FROM sqlite_schema WHERE type = 'table'")
                .fetch(pool);

        let table_infos = table_names
            .and_then(|name| TableInfo::new(name, pool))
            .try_collect::<Vec<_>>()
            .await?;

        Ok(Schema { raw, table_infos })
    }
}
