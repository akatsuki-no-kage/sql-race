use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use widgetui::{Chunks, Res, ResMut, State, WidgetFrame, WidgetResult};

pub struct Chunk;

#[derive(Default, State)]
pub struct CustomState {
    score: usize,
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
) -> WidgetResult {
    let chunk = chunks.get_chunk::<Chunk>()?;

    let score = Paragraph::new(state.score.to_string())
        .centered()
        .block(Block::default().title("Score").borders(Borders::ALL));

    frame.render_widget(score, chunk);

    Ok(())
}
