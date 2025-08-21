use sqlx::{Row, sqlite::SqliteRow};

fn is_cell_equal<'a, T: sqlx::Type<sqlx::Sqlite> + sqlx::Decode<'a, sqlx::Sqlite> + PartialEq>(
    index: usize,
    first: &'a SqliteRow,
    second: &'a SqliteRow,
) -> Option<bool> {
    match (
        first.try_get::<'_, T, _>(index),
        second.try_get::<'_, T, _>(index),
    ) {
        (Ok(first), Ok(second)) => Some(first == second),
        (Err(_), Ok(_)) | (Ok(_), Err(_)) => Some(false),
        _ => None,
    }
}

fn is_cell_equal_full(index: usize, first: &SqliteRow, second: &SqliteRow) -> bool {
    [
        is_cell_equal::<bool>(index, first, second),
        is_cell_equal::<i32>(index, first, second),
        is_cell_equal::<i64>(index, first, second),
        is_cell_equal::<f64>(index, first, second),
        is_cell_equal::<String>(index, first, second),
        is_cell_equal::<Vec<u8>>(index, first, second),
    ]
    .iter()
    .any(|&res| res == Some(true))
}

pub fn is_row_equal(first: &SqliteRow, second: &SqliteRow) -> bool {
    if first.len() != second.len() {
        return false;
    }
    let column_count = first.len();

    (0..column_count).all(|index| is_cell_equal_full(index, first, second))
}
