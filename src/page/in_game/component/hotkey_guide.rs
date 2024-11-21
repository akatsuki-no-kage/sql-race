use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct HotKeyGuild;

const HOTKEY: &str = "Ctrl+A: Select All / Ctrl + R: Run Query / Enter: Choose / \u{2192}, \u{2190}: Move / Esc: Menu";

impl Widget for HotKeyGuild {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hotkey_guide = Text::from(vec![Line::from(vec![HOTKEY.green().into()])]);
        Paragraph::new(hotkey_guide)
            .centered()
            .block(Block::default().borders(Borders::ALL))
            .render(area, buf);
    }
}
