use futures::{stream::FuturesOrdered, TryStreamExt};
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
    util,
};

pub struct Chunk;

const ID: usize = 1;
const QUESTION_COUNT: usize = 10;

#[derive(State)]
pub struct CustomState {
    pub questions: Vec<Question>,
    pub selected_question: usize,
}

impl Default for CustomState {
    fn default() -> Self {
        let questions: Vec<_> = util::run_async(async {
            (1..=QUESTION_COUNT)
                .map(util::get_question)
                .collect::<FuturesOrdered<_>>()
                .try_collect()
                .await
                .unwrap()
        });
        Self {
            questions,
            selected_question: Default::default(),
        }
    }
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
    focus_state: Res<FocusState>,
) -> WidgetResult {
    let Ok(chunk) = chunks.get_chunk::<Chunk>() else {
        return Ok(());
    };

    let border_color = if focus_state.focused_element == ID {
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
