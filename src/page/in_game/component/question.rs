use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::page::in_game::InGameState;

pub struct Question<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Question<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Text::from(self.in_game_state.question.question.as_str()))
            .block(Block::default().title("Question").borders(Borders::ALL))
            .scroll((0, 0))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .render(area, buf);
    }
}
