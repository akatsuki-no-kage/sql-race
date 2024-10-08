use anyhow::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table, Widget},
};
use sqlx::SqlitePool;

use crate::models::score::Score;

#[derive(Default)]
pub struct Ranking {
    pub scores: Vec<Score>,
}

impl Widget for &Ranking {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let header_height = Constraint::Percentage(5);
        let table_height = Constraint::Percentage(90);

        let table_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[header_height, table_height])
            .split(area);

        let header_block = Block::default()
            .title("Ranking")
            .title_alignment(Alignment::Center);
        let header = Paragraph::new("").block(header_block);

        let col_length = Constraint::Ratio(1, 3);

        let rows: Vec<Row> = self
            .scores
            .clone()
            .into_iter()
            .map(|score| score_into_row(score))
            .collect();

        let table_block = Block::default().borders(Borders::ALL);
        let table_body = Table::new(rows, vec![col_length, col_length, col_length])
            .header(Row::new(vec![
                text("Username")
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Yellow)),
                text("Score")
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Yellow)),
                text("Time")
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Yellow)),
            ]))
            .block(table_block);

        header.render(table_layout[0], buf);
        table_body.render(table_layout[1], buf);
    }
}

fn text(content: &str) -> Text<'static> {
    Text::from(content.to_owned()).alignment(Alignment::Center)
}

fn score_into_row(s: Score) -> Row<'static> {
    Row::new(vec![
        text(&s.username.clone()),
        text(&s.score.to_string()),
        text(&s.created_at.to_string()),
    ])
}

impl Ranking {
    pub async fn get_sorted_scores(&mut self, db: &SqlitePool) -> Result<()> {
        let mut scores = Score::get_all(db).await?;
        scores.sort_by(|score_prev, score_next| score_next.score.cmp(&score_prev.score));

        self.scores = scores.clone();

        Ok(())
    }
}
