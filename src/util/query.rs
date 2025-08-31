use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rusqlite::{Connection, types::Value};

pub type Row = Vec<Value>;

pub fn run(query: &str, schema: &str) -> rusqlite::Result<(Vec<String>, Vec<Row>)> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(schema)?;

    let mut stmt = conn.prepare(query)?;
    let column_count = stmt.column_count();

    let header = stmt
        .column_names()
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let rows = stmt
        .query_map((), |row| {
            (0..column_count)
                .map(|idx| row.get_ref(idx).map(Value::from))
                .collect::<Result<Row, _>>()
        })?
        .collect::<Result<_, _>>()?;

    Ok((header, rows))
}

pub fn is_equal(first: &str, second: &str, schema: &str) -> rusqlite::Result<bool> {
    let [first, second] = [first, second]
        .par_iter()
        .map(|&query| run(query, schema))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .unwrap();

    Ok(first == second)
}
