mod id;
mod message;
mod screen;
mod ui;
mod update;

use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use tuirealm::{
    Application, EventListenerCfg, NoUserEvent, Sub, SubClause, SubEventClause,
    application::ApplicationResult,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
};

use crate::{
    component::{
        editor::Editor, global_listener::GlobalListener, score_table::ScoreTable, timer::Timer,
        username_input::UsernameInput,
    },
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

        match self.screen {
            Screen::Home => {
                self.mount(
                    Id::UsernameInput,
                    Box::new(UsernameInput::default()),
                    Vec::new(),
                )
                .unwrap();

                let scores = repository::score::get_all(&self.config.database_file).unwrap();
                self.mount(
                    Id::ScoreTable,
                    Box::new(ScoreTable::new(scores)),
                    Vec::new(),
                )
                .unwrap();

                self.active(&Id::UsernameInput).unwrap();
            }
            Screen::Game => {
                let timer = Timer::new(
                    Duration::from_secs(self.config.game_duration),
                    Duration::from_secs(self.config.tick_rate),
                );
                self.mount(
                    Id::Timer,
                    Box::new(timer),
                    vec![Sub::new(SubEventClause::Tick, SubClause::Always)],
                )
                .unwrap();

                self.mount(Id::Editor, Box::new(Editor::default()), Vec::new())
                    .unwrap();

                self.active(&Id::Editor).unwrap();
            }
        }
    }

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

    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                ui::draw(&mut self.inner, self.screen, f);
            })
            .unwrap();
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
