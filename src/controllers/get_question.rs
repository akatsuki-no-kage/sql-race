use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::models::question::Question;

pub async fn get_question(path_dir: &Path) -> Result<Question> {
    let answer = fs::read_to_string(path_dir.join("answer.sql")).await?;
    let question = fs::read_to_string(path_dir.join("question.txt")).await?;
    let schema = fs::read_to_string(path_dir.join("schema.sql")).await?;

    Ok(Question {
        question,
        answer,
        schema,
    })
}
