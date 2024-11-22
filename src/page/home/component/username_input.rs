use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tokio::runtime::Handle;
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::{
    model::Score,
    page::{home::HomeState, in_game::InGameState},
    state::{GlobalState, Screen},
};

pub struct UsernameInput<'a> {
    pub home_state: &'a HomeState,
    pub global_state: &'a GlobalState,
}

const INVALID_USERNAME_MESSAGE: &str = "Username already exists!";

impl Widget for UsernameInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Input section
                Constraint::Length(3), // Error message section (below input)
            ])
            .split(area);

        self.global_state.username.render(layout[0], buf);

        if self.home_state.is_username_valid {
            return;
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

        error_paragraph.render(error_area, buf);
    }
}

pub fn event_handler(
    events: Res<Events>,
    mut home_state: ResMut<HomeState>,
    mut in_game_state: ResMut<InGameState>,
    mut global_state: ResMut<GlobalState>,
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
            let handle = Handle::current();
            let pool = global_state.pool.clone();

            let username = global_state.get_username();
            let is_user_existed = std::thread::spawn(move || {
                handle.block_on(async { Score::is_user_existed(&username, &pool).await })
            })
            .join()
            .unwrap()?;

            home_state.is_username_valid = !is_user_existed;
            if home_state.is_username_valid {
                global_state.screen = Screen::InGame;
                in_game_state.reset();
            }
        }
        Event::Key(key_event) if key_event.modifiers == KeyModifiers::NONE => {
            global_state.username.input(*key_event);
        }
        _ => {}
    }

    Ok(())
}
