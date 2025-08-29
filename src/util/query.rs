use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rusqlite::{Connection, types::Value};

pub type Row = Vec<Value>;

pub fn run(query: &str, schema: &str) -> rusqlite::Result<Vec<Row>> {
    let conn = Connection::open_in_memory()?;

    conn.execute(schema, ())?;

    let mut stmt = conn.prepare(query)?;
    let column_count = stmt.column_count();

    stmt.query_map((), |row| {
        (0..column_count)
            .map(|idx| row.get_ref(idx).map(|row| Value::from(row)))
            .collect()
    })?
    .collect()
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
