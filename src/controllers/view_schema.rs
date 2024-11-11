use anyhow::Result;
use sqlx::{Row, SqlitePool};

use crate::models::schema::{QuestionRow, QuestionTable};

pub async fn view_schemas(pool: &SqlitePool) -> Result<Vec<QuestionTable>> {
    let table_names = sqlx::query!("SELECT name FROM sqlite_schema WHERE type = 'table'")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.name.unwrap())
        .collect::<Vec<String>>();

    let mut question_tables = vec![];

    for name in table_names {
        let rows = sqlx::query(&format!("PRAGMA table_info({})", &name))
            .fetch_all(pool)
            .await?;
        let mut question_rows = vec![];

        for row in rows {
            let question_row = QuestionRow {
                col_id: row.get::<i32, _>("cid") as u32,
                name: row.get("name"),
                data_type: row.get("type"),
                not_null: row.get::<i32, _>("notnull") != 0, // Convert 0/1 to bool
                default_value: row.get("dflt_value"),        // Option<String> to handle NULL
                primary_key: row.get::<i32, _>("pk") != 0,   // Convert 0/1 to bool
            };

            question_rows.push(question_row);
        }
        question_tables.push(QuestionTable {
            name: name.clone(),
            rows: question_rows,
        });
    }

    Ok(question_tables)
}
