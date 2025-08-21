use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;

#[derive(Debug, Clone, PartialEq, PartialOrd, sqlx::FromRow)]
pub struct Score {
    pub id: i64,
    pub username: String,
    pub score: i64,
    pub created_at: NaiveDateTime,
}

impl Score {
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Score>> {
        let scores = sqlx::query_as!(Score, "SELECT * FROM scores ORDER BY score DESC")
            .fetch_all(pool)
            .await?;

        Ok(scores)
    }

    pub async fn insert(username: String, score: i64, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            "INSERT INTO scores (username, score) VALUES (?, ?)",
            username,
            score,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn is_user_existed(username: &str, pool: &SqlitePool) -> Result<bool> {
        let score = sqlx::query_scalar!("SELECT username FROM scores WHERE username = ?", username)
            .fetch_optional(pool)
            .await?;
        Ok(score.is_some())
    }
}
