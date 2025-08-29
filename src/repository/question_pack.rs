use std::{fs, io, path::Path};

use rand::seq::IteratorRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rusqlite::Connection;

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub is_primary_key: bool,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<Column>,
}

impl TableInfo {
    fn new(name: String, conn: &Connection) -> rusqlite::Result<Self> {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", &name))?;
        let columns = stmt
            .query_and_then((), |raw| {
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

pub type QuestionPack = (Vec<String>, Vec<String>, Vec<Schema>);

#[derive(Default)]
pub struct QuestionPackRepository;

impl QuestionPackRepository {
    pub fn get(&self, dir: &Path, sample_size: usize) -> io::Result<QuestionPack> {
        let mut question_dirs = fs::read_dir(dir)?
            .filter_map(|x| x.ok().map(|x| x.path()))
            .choose_multiple(&mut rand::rng(), sample_size);
        question_dirs.sort();

        let questions = question_dirs
            .clone()
            .par_iter()
            .map(|dir| fs::read_to_string(dir.join("question.txt")))
            .collect::<Result<_, io::Error>>()?;

        let answers = question_dirs
            .clone()
            .par_iter()
            .map(|dir| fs::read_to_string(dir.join("answer.sql")))
            .collect::<Result<_, io::Error>>()?;

        let schemas = question_dirs
            .par_iter()
            .map(|dir| {
                let raw_schema = fs::read_to_string(dir.join("schema.sql"))?;
                Schema::new(raw_schema).map_err(io::Error::other)
            })
            .collect::<Result<_, io::Error>>()?;

        Ok((questions, answers, schemas))
    }
}
