use std::time::Duration;

use anyhow::Result;
use ratatui::{
    crossterm::event,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};
use sqlx::SqlitePool;
use tui_textarea::{Input, Key};

use crate::{
    app::{App, AppState},
    controllers::check_exist_username,
    models::score::Score,
};

use super::components::{input::Input as UsernameInput, ranking::Ranking};

#[derive(Default)]
pub struct MenuPage {
    ranking: Ranking,
    pub input: UsernameInput,
    pub error: Option<String>,
}

impl Widget for &MenuPage {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Calculate the area for the input and error message
        let squarter_x = area.width / 4;
        let half_width = area.width / 2;

        let layout_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // Ranking section
                Constraint::Length(3),      // Input section
                Constraint::Length(3),      // Error message section (below input)
            ])
            .split(Rect::new(squarter_x, 0, half_width, area.height));

        // Render ranking section
        self.ranking.render(layout_vertical[0], buf);

        // Centered input section
        let input = UsernameInput::new("Enter name")
            .value(self.input.value.clone())
            .placeholder("Name");

        input.render(layout_vertical[1], buf);

        // Render error message below input (centered)
        if let Some(ref message) = self.error {
            let error_area = Rect::new(
                layout_vertical[2].x + (layout_vertical[2].width - message.len() as u16) / 2,
                layout_vertical[2].y,
                message.len() as u16,
                layout_vertical[2].height,
            );

            let error_paragraph =
                Paragraph::new(Text::styled(message, Style::default().fg(Color::Red)))
                    .block(Block::default().borders(Borders::NONE));

            error_paragraph.render(error_area, buf);
        }
    }
}

impl MenuPage {
    pub async fn load_scores(&mut self, db: &SqlitePool) -> Result<()> {
        self.ranking = Ranking { scores: Vec::new() };
        self.ranking.get_sorted_scores(db).await?;
        Ok(())
    }

    pub fn update_input(&mut self, new_value: String) {
        self.input.value = new_value;
    }

    pub fn clear_input(&mut self) {
        self.input.value.clear();
    }

    pub fn set_error_message(&mut self, message: String) {
        self.error = Some(message);
    }

    pub fn clear_error_message(&mut self) {
        self.error = None;
    }

    pub async fn handle_key_events(&mut self, app: &mut App) -> Result<()> {
        // Handle input events for ranking page
        let has_event = event::poll(Duration::from_millis(100))?;
        if !has_event {
            return Ok(());
        }
        match event::read()?.into() {
            Input {
                ctrl: true,
                key: Key::Char('q'),
                ..
            } => {
                app.exit = true;
            }

            Input {
                key: Key::Backspace,
                ..
            } => {
                if !self.input.value.is_empty() {
                    self.input.value.pop();
                }
            }
            Input {
                key: Key::Enter, ..
            } => {
                if check_exist_username(&app.pool, self.input.value.clone()).await? {
                    self.set_error_message("Username already exists!".to_string());
                } else {
                    app.username = self.input.value.clone();
                    Score::insert(&app.pool, app.username.clone(), app.score).await?;
                    app.state = AppState::InGame;
                };
            }
            Input {
                key: Key::Char(c), ..
            } => {
                self.update_input(format!("{}{}", self.input.value, c));
            }
            _ => {}
        }

        Ok(())
    }
}
