pub mod component;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};
use tui_textarea::TextArea;
use widgetui::{Events, Res, ResMut, State, WidgetResult};

use crate::{
    model::Score,
    state::{GlobalState, Screen},
};
use component::{rank::Rank, username_input::UsernameInput};

#[derive(State)]
pub struct HomeState {
    pub scores: Vec<Score>,
    pub is_username_valid: bool,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            scores: Default::default(),
            is_username_valid: true,
        }
    }
}

pub struct Home<'a> {
    pub home_state: Res<'a, HomeState>,
    pub global_state: Res<'a, GlobalState>,
}

impl Widget for Home<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let squarter_x = area.width / 4;
        let half_width = area.width / 2;

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // Ranking section
                Constraint::Length(6),      // Input section
            ])
            .split(Rect::new(squarter_x, 0, half_width, area.height));

        Rank {
            home_state: &self.home_state,
        }
        .render(layout[0], buf);

        UsernameInput {
            home_state: &self.home_state,
            global_state: &self.global_state,
        }
        .render(layout[1], buf);
    }
}

pub fn event_handler(mut events: ResMut<Events>, global_state: Res<GlobalState>) -> WidgetResult {
    if global_state.screen != Screen::Home {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => events.register_exit(),
        _ => {}
    }

    Ok(())
}
