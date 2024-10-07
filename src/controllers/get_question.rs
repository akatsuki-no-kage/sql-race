use anyhow::Result;
use tokio::fs;

use crate::models::question::Question;

pub async fn get_question(path_dir: String) -> Result<Question> {
    let answer = fs::read_to_string(format!("{}/answer.sql", path_dir)).await?;
    let question = fs::read_to_string(format!("{}/question.txt", path_dir)).await?;
    let schema = fs::read_to_string(format!("{}/schema.sql", path_dir)).await?;

    Ok(Question {
        question,
        answer,
        schema,
    })
}
