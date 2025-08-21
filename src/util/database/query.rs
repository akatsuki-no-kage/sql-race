use sqlx::{Executor, SqlitePool, sqlite::SqliteRow};

pub async fn run(query: &str, schema: &str) -> sqlx::Result<Vec<SqliteRow>> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    pool.execute(schema).await?;

    sqlx::query(query).fetch_all(&pool).await
}
