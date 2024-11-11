use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct Input {
    pub value: String,
    pub title: String,
    pub placeholder: String,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            value: String::new(),
            title: String::from("Your username"),
            placeholder: String::from("Type here..."),
        }
    }
}

impl Input {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.value.is_empty() {
            Text::styled(&self.placeholder, Style::default().fg(Color::DarkGray))
        } else {
            Text::raw(&self.value)
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
