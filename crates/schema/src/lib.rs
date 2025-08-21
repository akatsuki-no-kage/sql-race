use sqlx::SqlitePool;

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
pub struct Schema {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Schema {
    async fn from_name(name: String, pool: &SqlitePool) -> sqlx::Result<Schema> {
        let columns: Vec<Column> = sqlx::query_as(&format!("PRAGMA table_info({})", &name))
            .fetch_all(pool)
            .await?;

        Ok(Schema { name, columns })
    }
}
