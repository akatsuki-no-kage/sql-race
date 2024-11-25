use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;
use widgetui::{Chunks, Events, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    model::Score,
    page::in_game::InGameState,
    state::{GlobalState, Screen},
    util,
};

const INVALID_USERNAME_MESSAGE: &str = "Username already exists!";

#[derive(State)]
pub struct CustomState {
    pub username: TextArea<'static>,
    pub is_username_valid: bool,
}

impl Default for CustomState {
    fn default() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Name here");
        text_area.set_block(Block::default().borders(Borders::ALL).title("Name"));
        text_area.set_alignment(Alignment::Left);
        Self {
            username: text_area,
            is_username_valid: true,
        }
    }
}

impl CustomState {
    pub fn get_username(&self) -> String {
        self.username.lines().join("\n")
    }
}

pub struct Chunk;

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
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Input section
            Constraint::Length(3), // Error message section (below input)
        ])
        .split(chunk);

    state.username.render(layout[0], frame.buffer_mut());

    if state.is_username_valid {
        return Ok(());
    }

    let error_area = Rect::new(
        layout[1].x + (layout[1].width - INVALID_USERNAME_MESSAGE.len() as u16) / 2,
        layout[1].y,
        INVALID_USERNAME_MESSAGE.len() as u16,
        layout[1].height,
    );

    let error_paragraph = Paragraph::new(Text::styled(
        INVALID_USERNAME_MESSAGE,
        Style::default().fg(Color::Red),
    ))
    .block(Block::default().borders(Borders::NONE));

    error_paragraph.render(error_area, frame.buffer_mut());

    Ok(())
}

pub fn event_handler(
    events: Res<Events>,
    mut state: ResMut<CustomState>,
    mut global_state: ResMut<GlobalState>,
    mut in_game_state: ResMut<InGameState>,
) -> WidgetResult {
    if global_state.screen != Screen::Home {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            let pool = global_state.pool.clone();

            let username = state.get_username();
            let is_user_existed =
                util::run_async(async move { Score::is_user_existed(&username, &pool).await })?;

            state.is_username_valid = !is_user_existed;
            if state.is_username_valid {
                global_state.screen = Screen::InGame;
                in_game_state.reset();
            }
        }
        Event::Key(key_event) => {
            state.username.input(*key_event);
        }
        _ => {}
    }

    Ok(())
}
