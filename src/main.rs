use anyhow::Result;
use app::App;

pub mod app;
pub mod controllers;
pub mod models;
pub mod views;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new(String::new()).await?;
    views::init(&mut app).await?;
    Ok(())
}
