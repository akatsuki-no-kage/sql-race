use anyhow::Result;
use sqlx::SqlitePool;

pub async fn check_exist_username(pool: &SqlitePool, username: String) -> Result<bool> {
    let query = sqlx::query!("SELECT username FROM scores WHERE username = ?", username)
        .fetch_optional(pool)
        .await?;

    Ok(query.is_some())
}
