use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};
use sqlx::SqlitePool;

use super::components::{input::Input, ranking::Ranking};

#[derive(Default)]
pub struct RankingPage {
    ranking: Ranking,
    pub input: Input,
    pub error: Option<String>,
}

impl Widget for &RankingPage {
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
        let input = Input::new("Search")
            .value(self.input.value.clone())
            .placeholder("Type to search...");

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

impl RankingPage {
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
}
