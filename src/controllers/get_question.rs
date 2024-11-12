use anyhow::{Error, Result};
use std::path::Path;
use tokio::fs;

use crate::models::question::Question;

pub async fn get_question(quesiton_dir: &Path) -> Result<Question> {
    if quesiton_dir.is_dir() {
        let answer = fs::read_to_string(quesiton_dir.join("answer.sql")).await?;
        let question = fs::read_to_string(quesiton_dir.join("question.txt")).await?;
        let schema = fs::read_to_string(quesiton_dir.join("schema.sql")).await?;

        Ok(Question {
            question,
            answer,
            schema,
        })
    } else {
        Err(Error::msg(format!(
            "Folder {:?} is not found!",
            quesiton_dir
        )))
    }
}
