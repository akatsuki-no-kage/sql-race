use super::run_query;
use sqlx::{sqlite::SqliteRow, Result, Row};

// TODO: remove unwrap
fn is_row_equal(user_row: SqliteRow, answer_row: SqliteRow) -> bool {
    if user_row.len() != answer_row.len() {
        return false;
    }
    let column_count = user_row.len();

    (0..column_count).all(|column_index| {
        user_row.try_get::<String, _>(column_index).unwrap()
            == answer_row.try_get::<String, _>(column_index).unwrap()
    })
}

pub async fn get_score(user_query: &str, answer_query: &str, schema: &str) -> Result<bool> {
    let answer_rows = run_query(answer_query, schema).await?;
    let user_rows = run_query(user_query, schema).await?;

    if answer_rows.len() != user_rows.len() {
        return Ok(false);
    }

    Ok(user_rows
        .into_iter()
        .zip(answer_rows.into_iter())
        .all(|(user_row, answer_row)| is_row_equal(user_row, answer_row)))
}
