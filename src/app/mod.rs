mod id;
mod message;
mod state;
mod ui;

use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use tuirealm::{
    Application, EventListenerCfg, NoUserEvent, Sub, SubClause, SubEventClause, Update,
    application::ApplicationResult,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
};

use crate::{
    component::{
        global_listener::GlobalListener, score_table::ScoreTable, timer::Timer,
        username_input::UsernameInput,
    },
    config::CONFIG,
};

pub use id::*;
pub use message::*;
pub use state::*;

pub struct App<T>
where
    T: TerminalAdapter,
{
    pub inner: Application<Id, Message, NoUserEvent>,
    pub state: AppState,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge<T>,
}

impl<T> App<T>
where
    T: TerminalAdapter,
{
    fn mount_all(&mut self) {
        self.umount_all();

        self.mount(
            Id::GlobalListener,
            Box::new(GlobalListener::default()),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )
        .unwrap();

        match self.state.screen {
            Screen::Home => {
                self.mount(
                    Id::UsernameInput,
                    Box::new(UsernameInput::default()),
                    Vec::new(),
                )
                .unwrap();

                self.mount(Id::ScoreTable, Box::new(ScoreTable::default()), Vec::new())
                    .unwrap();

                self.active(&Id::UsernameInput).unwrap();
            }
            Screen::Game => {
                self.mount(
                    Id::Timer,
                    Box::new(Timer::new(Duration::from_secs(CONFIG.game_duration))),
                    vec![Sub::new(SubEventClause::Tick, SubClause::Always)],
                )
                .unwrap();

                self.active(&Id::Timer).unwrap();
            }
        }
    }

    fn active_next(&mut self) -> ApplicationResult<()> {
        let next = match self.inner.focus() {
            Some(Id::UsernameInput) => Id::ScoreTable,
            Some(Id::ScoreTable) => Id::UsernameInput,
            Some(current) => current.clone(),
            None => match self.state.screen {
                Screen::Home => Id::UsernameInput,
                Screen::Game => Id::Timer,
            },
        };

        self.active(&next)
    }

    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                ui::draw(&mut self.inner, self.state.screen, f);
            })
            .unwrap();
    }
}

impl<T> Update<Message> for App<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, message: Option<Message>) -> Option<Message> {
        let message = message?;
        self.redraw = true;

        match message {
            Message::Close => {
                self.quit = true;

                None
            }

            Message::Play(username) => {
                self.state.name = Some(username);

                Some(Message::ChangeScreen(Screen::Game))
            }
            Message::ChangeScreen(screen) => {
                self.state.screen = screen;
                if matches!(screen, Screen::Home) {
                    self.state = AppState::default();
                }

                self.mount_all();

                None
            }
            Message::ActiveNext => {
                self.active_next().unwrap();

                None
            }
            Message::None => None,
        }
    }
}

impl Default for App<CrosstermTerminalAdapter> {
    fn default() -> Self {
        let inner = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(CONFIG.tick_rate)),
        );

        let mut app = Self {
            inner,
            state: AppState::default(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().unwrap(),
        };

        app.mount_all();

        app
    }
}

impl<T: TerminalAdapter> Deref for App<T> {
    type Target = Application<Id, Message, NoUserEvent>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: TerminalAdapter> DerefMut for App<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
