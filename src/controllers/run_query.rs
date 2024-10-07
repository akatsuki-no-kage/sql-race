use sqlx::{sqlite::SqliteRow, Executor, Result, SqlitePool};

pub async fn run_query(query: &str, schema: &str) -> Result<Vec<SqliteRow>> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    pool.execute(schema).await?;

    sqlx::query(query).fetch_all(&pool).await
}
