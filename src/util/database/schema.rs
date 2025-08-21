use futures::TryStreamExt;
use sqlx::{Executor, SqlitePool};

#[derive(Debug, sqlx::FromRow)]
pub struct Column {
    #[sqlx(rename = "cid")]
    pub name: String,

    #[sqlx(rename = "type")]
    pub data_type: String,

    #[sqlx(rename = "pk")]
    pub is_primary_key: bool,

    #[sqlx(rename = "notnull")]
    pub is_not_null: bool,

    #[sqlx(rename = "dflt_value")]
    pub default_value: Option<String>,
}

#[derive(Debug)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<Column>,
}

impl TableInfo {
    pub async fn new(name: String, pool: &SqlitePool) -> sqlx::Result<TableInfo> {
        let columns: Vec<Column> = sqlx::query_as(&format!("PRAGMA table_info({})", &name))
            .fetch_all(pool)
            .await?;

        Ok(TableInfo { name, columns })
    }
}

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
