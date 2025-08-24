use bevy::ecs::system::{Query, Res, ResMut};
use bevy_ratatui::RatatuiContext;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::score_board::{Chunk, Score};

fn into_row<'a, const N: usize>(data: [String; N]) -> Row<'a> {
    Row::new(data.map(|content| Text::from(content).alignment(Alignment::Center)))
}

const HEADERS: [&str; 3] = ["Username", "Score", "Time"];

pub fn render(scores: Query<&Score>, chunk: Res<Chunk>, mut context: ResMut<RatatuiContext>) {
    let header = into_row(HEADERS.map(|x| x.to_string()));
    let scores = scores
        .iter()
        .map(|score| {
            [
                score.username.clone(),
                score.score.to_string(),
                score.created_at.to_string(),
            ]
        })
        .map(into_row);

    let title_block = Block::default()
        .title("Ranking")
        .title_alignment(Alignment::Center);
    let title = Paragraph::new("").block(title_block);

    let table_block = Block::default().borders(Borders::ALL);
    let table_body = Table::new(scores, [Constraint::Ratio(1, 3); 3])
        .header(header)
        .block(table_block);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(90)])
        .split(chunk.into_inner().0);

    context.draw(|frame| {
        frame.render_widget(title, layout[0]);
        frame.render_widget(table_body, layout[1]);
    });
}
