use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::model::Question;

pub async fn get_question(question_index: usize) -> Result<Question> {
    let question_dir_path = format!("./questions/question-{}", question_index);
    let question_dir = Path::new(&question_dir_path);
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
