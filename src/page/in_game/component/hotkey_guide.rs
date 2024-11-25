use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};
use widgetui::{Chunks, Res, ResMut, WidgetFrame, WidgetResult};

use crate::state::{GlobalState, Screen};

pub struct Chunk;

const HOTKEY: &str = "Ctrl+A: Select All / Ctrl + R: Run Query / Enter: Choose / \u{2192}, \u{2190}: Move / Esc: Menu";

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    let content = Text::from(vec![Line::from(vec![HOTKEY.green().into()])]);
    let hotkey_guide = Paragraph::new(content)
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(hotkey_guide, chunk);

    Ok(())
}
