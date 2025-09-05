use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuestionConfig {
    pub root: PathBuf,
    pub count: usize,
}
