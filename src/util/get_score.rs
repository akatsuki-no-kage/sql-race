use super::run_query;
use sqlx::{sqlite::SqliteRow, Result, Row};

// TODO: remove unwrap
fn is_row_equal(user_row: SqliteRow, answer_row: SqliteRow) -> bool {
    if user_row.len() != answer_row.len() {
        return false;
    }
    let column_count = user_row.len();

    (0..column_count).all(|column_index| {
        let user_value = user_row
            .try_get::<String, _>(column_index)
            .map(|val| val)
            .unwrap_or_else(|_| {
                user_row
                    .try_get::<i64, _>(column_index)
                    .map(|val| val.to_string())
                    // If both String and i64 fail, return a default (like "NULL")
                    .unwrap_or_else(|_| "NULL".to_string())
            });

        let answer_value = answer_row
            .try_get::<String, _>(column_index)
            .map(|val| val)
            .unwrap_or_else(|_| {
                answer_row
                    .try_get::<i64, _>(column_index)
                    .map(|val| val.to_string())
                    .unwrap_or_else(|_| "NULL".to_string())
            });
        user_value == answer_value
    })
}

pub async fn get_score(user_query: &str, answer_query: &str, schema: &str) -> Result<u32> {
    let answer_rows = run_query(answer_query, schema).await?;
    let user_rows = run_query(user_query, schema).await?;

    if answer_rows.len() != user_rows.len() {
        return Ok(0);
    }

    let is_all_match = user_rows
        .into_iter()
        .zip(answer_rows.into_iter())
        .all(|(user_row, answer_row)| is_row_equal(user_row, answer_row));
    Ok(if is_all_match { 1 } else { 0 })
}