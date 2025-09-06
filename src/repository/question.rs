use std::collections::HashMap;

use rusqlite::Connection;
use serde::Deserialize;

use crate::config::{CONFIG, Mode};

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub is_primary_key: bool,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<Column>,
}

impl TableInfo {
    fn new(name: String, conn: &Connection) -> rusqlite::Result<Self> {
        let mut stmt = conn.prepare("SELECT * FROM pragma_table_info(?)")?;
        let columns = stmt
            .query_and_then([&name], |raw| {
                Ok::<_, rusqlite::Error>(Column {
                    name: raw.get("name")?,
                    is_primary_key: raw.get("pk")?,
                    data_type: raw.get("type")?,
                    is_nullable: raw.get("notnull")?,
                    default_value: raw.get("dflt_value")?,
                })
            })?
            .collect::<Result<_, _>>()?;

        Ok(Self { name, columns })
    }
}

#[derive(Debug)]
pub struct Schema {
    pub raw: String,
    pub table_infos: Vec<TableInfo>,
}

impl Schema {
    pub fn new(raw: String) -> rusqlite::Result<Self> {
        let conn = Connection::open_in_memory()?;

        conn.execute_batch(&raw)?;

        let mut stmt = conn.prepare("SELECT name FROM sqlite_schema WHERE type = 'table'")?;
        let table_infos = stmt
            .query_and_then((), |raw| {
                raw.get::<_, String>("name")
                    .and_then(|name| TableInfo::new(name, &conn))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { raw, table_infos })
    }
}

#[derive(Debug, Deserialize)]
struct RawQuestion {
    question: String,
    answer: String,
    schema: String,
}

type QuestionPack = HashMap<Mode, Vec<RawQuestion>>;

#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub schema: Schema,
}

impl TryFrom<RawQuestion> for Question {
    type Error = rusqlite::Error;

    fn try_from(raw: RawQuestion) -> Result<Self, Self::Error> {
        let schema = Schema::new(raw.schema)?;

        Ok(Self {
            question: raw.question,
            answer: raw.answer,
            schema,
        })
    }
}

pub fn get_all() -> rusqlite::Result<Vec<Question>> {
    let mut question_pack: QuestionPack = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .and_then(|x| x.try_deserialize())
        .unwrap();

    let questions = question_pack.remove(&CONFIG.mode).unwrap();

    questions.into_iter().map(Question::try_from).collect()
}
