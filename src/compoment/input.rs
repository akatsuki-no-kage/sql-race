use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct Input<'a> {
    pub value: &'a str,
    pub title: &'a str,
    pub placeholder: &'a str,
}

impl Default for Input<'_> {
    fn default() -> Self {
        Self {
            value: "",
            title: "Input",
            placeholder: "Input here",
        }
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.value.is_empty() {
            Text::styled(self.placeholder, Style::default().fg(Color::DarkGray))
        } else {
            Text::raw(self.value)
        };

        let input = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.clone()),
            )
            .alignment(Alignment::Left);

        input.render(area, buf);
    }
}
