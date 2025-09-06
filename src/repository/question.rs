use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rand::seq::IteratorRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use rusqlite::Connection;

use crate::config::{CONFIG, question::Level};

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

#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub schema: Schema,
}

fn get_paths(level: Level, count: usize) -> io::Result<Vec<PathBuf>> {
    let level: &str = level.into();

    let mut question_dirs = fs::read_dir(CONFIG.question.root.join(level))?
        .filter_map(|x| x.ok().map(|x| x.path()))
        .choose_multiple(&mut rand::rng(), count);
    question_dirs.sort_by_key(|x| {
        x.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });

    Ok(question_dirs)
}

fn get_all_paths() -> impl Iterator<Item = PathBuf> {
    [Level::Easy, Level::Medium, Level::Hard]
        .map(|level| get_paths(level, CONFIG.question.count[&level]).unwrap())
        .into_iter()
        .flatten()
}

pub fn get_all() -> io::Result<Vec<Question>> {
    let question_dirs = get_all_paths();

    question_dirs
        .map(|dir| {
            let question = fs::read_to_string(dir.join("question.txt"))?;

            let answer = fs::read_to_string(dir.join("answer.sql"))?;

            let raw_schema = fs::read_to_string(dir.join("schema.sql"))?;
            let schema = Schema::new(raw_schema).map_err(io::Error::other)?;

            Ok::<_, io::Error>(Question {
                question,
                answer,
                schema,
            })
        })
        .collect()
}
