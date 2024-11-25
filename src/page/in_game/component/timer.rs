use std::time::{Duration, Instant};

use ratatui::widgets::{Block, Borders, Paragraph};
use widgetui::{Chunks, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::state::{GlobalState, Screen};

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
