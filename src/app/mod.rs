mod id;
mod message;
mod mount;
mod screen;
mod ui;
mod update;

use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use tuirealm::{
    Application, EventListenerCfg, NoUserEvent,
    application::ApplicationResult,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
};

use crate::{
    config::Config,
    repository::{self, question::Question},
};

pub use id::*;
pub use message::*;
pub use screen::*;

pub struct App<T>
where
    T: TerminalAdapter,
{
    pub inner: Application<Id, Message, NoUserEvent>,

    pub username: Option<String>,

    pub config: Config,
    pub questions: Vec<Question>,
    pub question_index: usize,

    pub screen: Screen,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge<T>,
}

impl<T> App<T>
where
    T: TerminalAdapter,
{
    fn active_next(&mut self) -> ApplicationResult<()> {
        let next = match self.inner.focus() {
            Some(Id::UsernameInput) => Some(Id::ScoreTable),
            Some(Id::ScoreTable) => Some(Id::UsernameInput),
            None => Some(match self.screen {
                Screen::Home => Id::UsernameInput,
                Screen::Game => Id::Editor,
            }),
            _ => None,
        };

        if let Some(next) = next {
            self.active(&next)
        } else {
            Ok(())
        }
    }
}

impl Default for App<CrosstermTerminalAdapter> {
    fn default() -> Self {
        let config = Config::new().unwrap();

        let inner = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(config.tick_rate)),
        );

        let questions =
            repository::question::get_many(&config.question_pack_dir, config.question_count)
                .unwrap();

        let mut app = Self {
            inner,

            username: None,

            config,
            questions,
            question_index: 0,

            screen: Screen::Home,
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
