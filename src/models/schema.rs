#[derive(Debug)]
pub struct QuestionRow {
    pub col_id: u32,
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub default_value: Option<String>, // Nullable in the schema
    pub primary_key: bool,
}

#[derive(Debug)]
pub struct QuestionTable {
    pub name: String,
    pub rows: Vec<QuestionRow>,
}
