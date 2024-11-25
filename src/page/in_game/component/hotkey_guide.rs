use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};
use widgetui::{Chunks, Res, ResMut, WidgetFrame, WidgetResult};

pub struct Chunk;

const HOTKEY: &str = "Ctrl+A: Select All / Ctrl + R: Run Query / Enter: Choose / \u{2192}, \u{2190}: Move / Esc: Menu";

pub fn render(mut frame: ResMut<WidgetFrame>, chunks: Res<Chunks>) -> WidgetResult {
    let Ok(chunk) = chunks.get_chunk::<Chunk>() else {
        return Ok(());
    };

    let content = Text::from(vec![Line::from(vec![HOTKEY.green()])]);
    let hotkey_guide = Paragraph::new(content)
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(hotkey_guide, chunk);

    Ok(())
}
