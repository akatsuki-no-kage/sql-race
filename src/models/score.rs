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
    pub async fn get_all(db: &SqlitePool) -> Result<Vec<Score>> {
        let scores = sqlx::query_as!(Score, "SELECT * FROM scores")
            .fetch_all(db)
            .await?;

        Ok(scores)
    }

    pub async fn insert(&self, db: &SqlitePool) -> Result<()> {
        sqlx::query("INSERT INTO scores (username, score, created_at) VALUES (?, ?, ?)")
            .bind(&self.username)
            .bind(&self.score)
            .bind(&self.created_at)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn delete_all(pool: &SqlitePool) -> Result<()> {
        sqlx::query("DELETE FROM scores").execute(pool).await?;

        Ok(())
    }

    pub async fn delete_one(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM scores
            WHERE username = ?
            "#,
        )
        .bind(&self.username)
        .execute(pool)
        .await?;

        Ok(())
    }
}
