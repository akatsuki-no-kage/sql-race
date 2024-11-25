use ratatui::{
    layout::{Alignment, Constraint},
    prelude::*,
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table},
};
use widgetui::{constraint, layout, Chunks, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    model::Score,
    state::{GlobalState, Screen},
    util,
};

#[derive(State, Default)]
pub struct CustomState {
    pub scores: Vec<Score>,
}

pub fn state_updater(
    mut state: ResMut<CustomState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    let pool = global_state.pool.clone();

    let scores = util::run_async(async move { Score::get_all(&pool).await })?;
    state.scores = scores;

    Ok(())
}

pub struct Chunk;

fn into_row<'a, const N: usize>(data: [String; N]) -> Row<'a> {
    Row::new(data.map(|content| Text::from(content).alignment(Alignment::Center)))
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::Home {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    let sub_chunks = layout! {
        chunk,
        (%5),
        (%90)
    };

    let header_block = Block::default()
        .title("Ranking")
        .title_alignment(Alignment::Center);
    let header = Paragraph::new("").block(header_block);

    let col_length = Constraint::Ratio(1, 3);

    let headers = into_row(["Username", "Score", "Time"].map(|s| s.to_string()));
    let rows: Vec<_> = state
        .scores
        .iter()
        .map(|score| {
            into_row([
                score.username.clone(),
                score.score.to_string(),
                score.created_at.to_string(),
            ])
        })
        .collect();

    let table_block = Block::default().borders(Borders::ALL);
    let table_body = Table::new(rows, vec![col_length, col_length, col_length])
        .header(headers)
        .block(table_block);

    frame.render_widget(header, sub_chunks[0][0]);
    frame.render_widget(table_body, sub_chunks[1][0]);

    Ok(())
}
