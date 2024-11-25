use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::page::in_game::InGameState;

pub struct Chunk;

const ID: usize = 1;

pub struct Question<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Question<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = if self.in_game_state.focused_element == ID {
            Color::Green
        } else {
            Color::White
        };

        Paragraph::new(Text::from(
            self.in_game_state.questions[self.in_game_state.question_index]
                .question
                .as_str(),
        ))
        .block(
            Block::default()
                .title("Question")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .scroll((0, 0))
        .wrap(ratatui::widgets::Wrap { trim: true })
        .render(area, buf);
    }
}
