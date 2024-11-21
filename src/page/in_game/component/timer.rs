use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::page::in_game::InGameState;

pub struct Timer<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Timer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let time_left = self.in_game_state.get_time_left();

        Paragraph::new(time_left.as_secs().to_string())
            .centered()
            .block(Block::default().title("Time left").borders(Borders::ALL))
            .render(area, buf);
    }
}
