use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use widgetui::{Chunks, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    model::Question,
    page::in_game::FocusState,
    state::{GlobalState, Screen},
};

pub struct Chunk;

const ID: usize = 1;

#[derive(Default, State)]
pub struct CustomState {
    pub questions: Vec<Question>,
    pub selected_question: usize,
}

impl CustomState {
    pub fn next(&mut self) {
        self.selected_question = (self.selected_question + 1) % self.questions.len();
    }

    pub fn prev(&mut self) {
        let length = self.questions.len();
        self.selected_question = (self.selected_question + length - 1) % length;
    }
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
    in_game_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    let border_color = if in_game_state.focused_element == ID {
        Color::Green
    } else {
        Color::White
    };

    let question = Paragraph::new(Text::from(
        state.questions[state.selected_question].question.as_str(),
    ))
    .block(
        Block::default()
            .title("Question")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    )
    .scroll((0, 0))
    .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(question, chunk);

    Ok(())
}
