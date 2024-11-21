use anyhow::Result;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};
use sqlx::SqlitePool;
use tokio::runtime::Handle;
use widgetui::{Events, Res, ResMut, State, WidgetResult};

use crate::{
    compoment::{input::Input, rank::Rank},
    model::Score,
    state::{GlobalState, Screen},
};

#[derive(State)]
pub struct HomeState {
    pub scores: Vec<Score>,
    pub error: Option<String>,
}

impl HomeState {
    pub async fn new(pool: &SqlitePool) -> Result<Self> {
        let scores = Score::get_all(pool).await?;

        Ok(Self {
            scores,
            error: None,
        })
    }
}

pub fn score_update(
    global_state: Res<GlobalState>,
    mut home_state: ResMut<HomeState>,
) -> WidgetResult {
    let handle = Handle::current();
    let pool = global_state.pool.clone();

    let scores = std::thread::spawn(move || handle.block_on(async { Score::get_all(&pool).await }))
        .join()
        .unwrap()?;
    home_state.scores = scores;
    Ok(())
}

pub fn handle_key(
    mut global_state: ResMut<GlobalState>,
    mut home_state: ResMut<HomeState>,
    mut events: ResMut<Events>,
) -> WidgetResult {
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
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        }) => global_state.screen = Screen::InGame,
        Event::Key(key_event) => {
            global_state.username.input(*key_event);
        }
        _ => {}
    }

    Ok(())
}

pub struct Home<'a> {
    pub global_state: Res<'a, GlobalState>,
    pub state: Res<'a, HomeState>,
}

impl Widget for Home<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Calculate the area for the input and error message
        let squarter_x = area.width / 4;
        let half_width = area.width / 2;

        let layout_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // Ranking section
                Constraint::Length(3),      // Input section
                Constraint::Length(3),      // Error message section (below input)
            ])
            .split(Rect::new(squarter_x, 0, half_width, area.height));

        // Render ranking section
        let rank_component = Rank {
            scores: &self.state.scores,
        };
        rank_component.render(layout_vertical[0], buf);

        // Centered input section
        self.global_state.username.render(layout_vertical[1], buf);

        // Render error message below input (centered)
        if let Some(ref message) = &self.state.error {
            let error_area = Rect::new(
                layout_vertical[2].x + (layout_vertical[2].width - message.len() as u16) / 2,
                layout_vertical[2].y,
                message.len() as u16,
                layout_vertical[2].height,
            );

            let error_paragraph =
                Paragraph::new(Text::styled(message, Style::default().fg(Color::Red)))
                    .block(Block::default().borders(Borders::NONE));

            error_paragraph.render(error_area, buf);
        }
    }
}
