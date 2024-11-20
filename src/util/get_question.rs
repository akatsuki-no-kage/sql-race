use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::model::Question;

pub async fn get_question(question_dir: &Path) -> Result<Question> {
    anyhow::ensure!(
        question_dir.is_dir(),
        "Folder {:?} is not found!",
        question_dir
    );

    let answer = fs::read_to_string(question_dir.join("answer.sql")).await?;
    let question = fs::read_to_string(question_dir.join("question.txt")).await?;
    let raw_schema = fs::read_to_string(question_dir.join("schema.sql")).await?;

    Question::new(question, answer, raw_schema).await
}
