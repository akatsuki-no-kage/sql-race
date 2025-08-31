mod id;
mod message;
mod screen;

use std::time::Duration;

use ratatui::layout::{Constraint, Layout, Rect};
use tuirealm::{
    Application, Component, EventListenerCfg, NoUserEvent, Sub, SubClause, SubEventClause, Update,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
};

use crate::{
    component::{
        Editor, GlobalListener, Help, QueryError, Question, ResultTable, Score, ScoreTable, Timer,
        UsernameInput,
    },
    config::Config,
    repository, util,
};

pub use id::*;
pub use message::*;
pub use screen::*;

pub struct App<T: TerminalAdapter> {
    pub inner: Application<Id, Message, NoUserEvent>,

    pub username: Option<String>,

    pub config: Config,
    pub questions: Vec<repository::question::Question>,
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

        app.change_screen(Screen::Home);

        app
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
            Message::Quit => self.quit(),
            Message::ToggleHelp => self.toggle_help(),
            Message::Start(username) => self.start(username),
            Message::Run => self.run(),
            Message::NextQuestion => self.next_question(),
            Message::End => self.end(),
            Message::ChangeScreen(screen) => self.change_screen(screen),
            Message::Active(offset) => self.active(offset),
            Message::None => None,
        }
    }
}

impl<T: TerminalAdapter> App<T> {
    fn get_components(focus: Option<&Id>, screen: Screen, area: Rect) -> Vec<(Id, Rect)> {
        if focus == Some(&Id::Help) {
            let chunks =
                Layout::horizontal([Constraint::Min(0), Constraint::Max(80), Constraint::Min(0)])
                    .split(area);
            let chunks =
                Layout::vertical([Constraint::Min(0), Constraint::Max(80), Constraint::Min(0)])
                    .split(chunks[1]);

            return vec![(Id::Help, chunks[1])];
        }

        match screen {
            Screen::Home => {
                let margined_chunks = Layout::horizontal([
                    Constraint::Min(0),
                    Constraint::Max(80),
                    Constraint::Min(0),
                ])
                .margin(2)
                .split(area);

                let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)])
                    .split(margined_chunks[1]);

                vec![(Id::ScoreTable, chunks[0]), (Id::UsernameInput, chunks[1])]
            }
            Screen::Game => {
                let chunks =
                    Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);
                let header_chunks = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Percentage(80),
                ])
                .split(chunks[0]);
                let content_chunks =
                    Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                        .split(chunks[1]);
                let editor_chunks =
                    Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                        .split(content_chunks[0]);

                vec![
                    (Id::Timer, header_chunks[0]),
                    (Id::Score, header_chunks[1]),
                    (Id::Editor, editor_chunks[0]),
                    (Id::Result, editor_chunks[1]),
                    (Id::Question, content_chunks[1]),
                ]
            }
        }
    }

    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                let components = Self::get_components(self.inner.focus(), self.screen, f.area());

                for (id, chunk) in components {
                    self.inner.view(&id, f, chunk);
                }
            })
            .unwrap();
    }

    fn toggle_help(&mut self) -> Option<Message> {
        match self.inner.focus() {
            Some(id) if id == &Id::Help => self.inner.blur().unwrap(),
            _ => self.inner.active(&Id::Help).unwrap(),
        }

        Some(Message::None)
    }

    fn start(&mut self, username: String) -> Option<Message> {
        self.username = Some(username);
        self.question_index = 0;

        Some(Message::ChangeScreen(Screen::Game))
    }

    fn run(&mut self) -> Option<Message> {
        let schema = self.current_question().schema.raw.as_str();
        let query = self
            .inner
            .state(&Id::Editor)
            .unwrap()
            .unwrap_one()
            .unwrap_string();

        let component: Box<dyn Component<_, _>> = match util::query::run(&query, schema) {
            Ok(data) => Box::new(ResultTable::new(Some(data))),
            Err(error) => Box::new(QueryError::new(error)),
        };

        self.inner
            .remount(Id::Result, component, Vec::new())
            .unwrap();

        Some(Message::None)
    }

    fn next_question(&mut self) -> Option<Message> {
        self.question_index += 1;

        if self.question_index == self.questions.len() {
            return Some(Message::End);
        }

        self.remount(Id::Editor);
        self.remount(Id::Question);
        self.remount(Id::Result);

        Some(Message::None)
    }

    fn end(&mut self) -> Option<Message> {
        let Some(username) = self.username.as_ref() else {
            return Some(Message::Quit);
        };

        repository::score::insert(
            username,
            self.question_index as u64,
            &self.config.database_file,
        )
        .unwrap();

        self.username = None;
        self.question_index = 0;

        Some(Message::ChangeScreen(Screen::Home))
    }

    fn remount(&mut self, id: Id) {
        let (component, subs): (Box<dyn Component<_, _>>, _) = match id {
            Id::GlobalListener => (
                Box::new(GlobalListener::default()),
                vec![Sub::new(SubEventClause::Any, SubClause::Always)],
            ),

            Id::Help => (Box::new(Help::default()), Vec::new()),

            Id::ScoreTable => {
                let scores = repository::score::get_all(&self.config.database_file).unwrap();

                (Box::new(ScoreTable::new(scores)), Vec::new())
            }

            Id::UsernameInput => (Box::new(UsernameInput::default()), Vec::new()),

            Id::Timer => (
                Box::new(Timer::new(
                    Duration::from_secs(self.config.game_duration),
                    Duration::from_secs(self.config.tick_rate),
                )),
                vec![Sub::new(SubEventClause::Tick, SubClause::Always)],
            ),

            Id::Score => (Box::new(Score::new(self.question_index as u64)), Vec::new()),

            Id::Question => (
                Box::new(Question::new(self.current_question().question.clone())),
                Vec::new(),
            ),

            Id::Result => (Box::new(ResultTable::new(None)), Vec::new()),

            Id::Editor => (Box::new(Editor::default()), Vec::new()),
        };

        self.inner.remount(id, component, subs).unwrap();
    }

    fn change_screen(&mut self, screen: Screen) -> Option<Message> {
        self.screen = screen;

        self.inner.umount_all();

        self.remount(Id::GlobalListener);
        self.remount(Id::Help);

        match screen {
            Screen::Home => {
                self.remount(Id::ScoreTable);
                self.remount(Id::UsernameInput);

                self.inner.active(&Id::UsernameInput).unwrap();
            }
            Screen::Game => {
                self.remount(Id::Timer);
                self.remount(Id::Score);
                self.remount(Id::Question);
                self.remount(Id::Result);
                self.remount(Id::Editor);

                self.inner.active(&Id::Editor).unwrap();
            }
        }

        Some(Message::None)
    }

    fn active(&mut self, offset: isize) -> Option<Message> {
        let active_list = match self.screen {
            Screen::Home => [Id::ScoreTable, Id::UsernameInput].as_slice(),
            Screen::Game => [Id::Editor, Id::Result, Id::Question].as_slice(),
        };
        let count = active_list.len() as isize;

        let active_index = self
            .inner
            .focus()
            .and_then(|id| active_list.iter().position(|x| x == id))
            .unwrap_or(0) as isize;

        let next_index = (active_index + count + offset) % count;

        self.inner
            .active(&active_list[next_index as usize])
            .unwrap();

        Some(Message::None)
    }

    fn quit(&mut self) -> Option<Message> {
        self.quit = true;

        None
    }

    fn current_question(&self) -> &repository::question::Question {
        &self.questions[self.question_index]
    }
}
