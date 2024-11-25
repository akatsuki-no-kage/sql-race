use std::time::{Duration, Instant};

use ratatui::widgets::{Block, Borders, Paragraph};
use widgetui::{Chunks, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    page::{home::component::username_input, in_game::finish_game},
    state::{GlobalState, Screen},
};

use super::score;

pub struct Chunk;

const TIME_OUT: Duration = Duration::from_secs(180);

#[derive(State)]
pub struct CustomState {
    end_time: Instant,
}

impl Default for CustomState {
    fn default() -> Self {
        Self {
            end_time: Instant::now() + TIME_OUT,
        }
    }
}

impl CustomState {
    fn get_time_left(&self) -> Duration {
        self.end_time.saturating_duration_since(Instant::now())
    }
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

    let time_left = state.get_time_left();

    let timer = Paragraph::new(time_left.as_secs().to_string())
        .centered()
        .block(Block::default().title("Time left").borders(Borders::ALL));

    frame.render_widget(timer, chunk);

    Ok(())
}

pub fn state_updater(
    state: Res<CustomState>,
    username_input_state: Res<username_input::CustomState>,
    score_state: Res<score::CustomState>,
    mut global_state: ResMut<GlobalState>,
) -> WidgetResult {
    if state.get_time_left() == Duration::ZERO {
        finish_game(&username_input_state, &score_state, &mut global_state);
    }

    Ok(())
}
