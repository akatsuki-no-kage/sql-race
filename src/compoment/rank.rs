use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table, Widget},
};

use crate::model::Score;

fn text<'a>(content: String) -> Text<'a> {
    Text::from(content.to_owned()).alignment(Alignment::Center)
}

impl Score {
    fn into_row<'a>(&'a self) -> Row<'a> {
        let row_content = [
            self.username.clone(),
            self.score.to_string(),
            self.created_at.to_string(),
        ];
        let row = row_content.map(text);

        Row::new(row)
    }
}

#[derive(Default)]
pub struct Rank<'a> {
    scores: &'a [Score],
}

impl<'a> Rank<'a> {
    pub fn scores(mut self, scores: &'a [Score]) -> Self {
        self.scores = scores;
        self
    }
}

impl Widget for Rank<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
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

        let rows: Vec<_> = self.scores.iter().map(|score| score.into_row()).collect();

        let table_block = Block::default().borders(Borders::ALL);
        let table_body = Table::new(rows, vec![col_length, col_length, col_length])
            .header(Row::new(vec![
                text("Username".to_string())
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Yellow)),
                text("Score".to_string())
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Yellow)),
                text("Time".to_string())
                    .add_modifier(Modifier::BOLD)
                    .style(Style::default().fg(Color::Yellow)),
            ]))
            .block(table_block);

        header.render(table_layout[0], buf);
        table_body.render(table_layout[1], buf);
    }
}
