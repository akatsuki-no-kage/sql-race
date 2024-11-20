use ratatui::widgets::{Paragraph, Widget};
use widgetui::Res;

use crate::state::GlobalState;

pub struct InGame<'a> {
    pub global_state: Res<'a, GlobalState>,
}

impl Widget for InGame<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.global_state.username.as_str()).render(area, buf)
    }
}
