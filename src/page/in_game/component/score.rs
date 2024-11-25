use ratatui::widgets::{Block, Borders, Paragraph};
use widgetui::{Chunks, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::state::{GlobalState, Screen};

pub struct Chunk;

#[derive(Default, State)]
pub struct CustomState {
    pub score: usize,
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    let score = Paragraph::new(state.score.to_string())
        .centered()
        .block(Block::default().title("Score").borders(Borders::ALL));

    frame.render_widget(score, chunk);

    Ok(())
}
