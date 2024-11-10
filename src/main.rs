use std::path::Path;

use anyhow::Result;
use app::App;
use controllers::get_question;
use sqlx::SqlitePool;

pub mod app;
pub mod controllers;
pub mod models;
pub mod views;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:score.db").await?;
    let mut app = App::new();
    views::init(&pool, &mut app).await?;

    println!("Hello, world!");
    Ok(())
}
