use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::page::in_game::InGameState;

pub struct Score<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Score<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.in_game_state.score.to_string())
            .centered()
            .block(Block::default().title("Score").borders(Borders::ALL))
            .render(area, buf);
    }
}
