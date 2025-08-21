use crate::Schema;

pub struct Database {
    pub schema: Schema,
}

impl Database {
    pub async fn new(raw_schema: String) -> sqlx::Result<Database> {
        let schema = Schema::new(raw_schema).await?;

        Ok(Database { schema })
    }
}
