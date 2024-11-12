use sqlx::{sqlite::SqliteRow, Error, Executor, Result, SqlitePool};

pub async fn run_query(query: &str, schema: &str) -> Result<Vec<SqliteRow>,Error> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    pool.execute(schema).await?;

    match sqlx::query(query).fetch_all(&pool).await {
        Ok(rows) => Ok(rows),
        Err(e) => {
            Err(e)
        }
    }
}
