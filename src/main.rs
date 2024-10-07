use anyhow::Result;
use controllers::get_question;

pub mod controllers;
pub mod models;
pub mod views;

#[tokio::main]
async fn main() -> Result<()> {
    let path = "./questions/qs_1";

    let question_data = get_question(path.to_string()).await?;
    println!("{:?}", question_data);

    Ok(())
}
