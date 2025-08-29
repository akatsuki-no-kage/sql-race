mod event;
mod id;
mod message;
mod screen;

use std::{sync::mpsc, time::Duration};

use tuirealm::{
    Application, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
};

use crate::{
    component::{name_input::NameInput, quit_listener::QuitListener, timer::Timer},
    config::CONFIG,
};

pub use event::*;
pub use id::*;
pub use message::*;
pub use screen::*;

pub struct App<T>
where
    T: TerminalAdapter,
{
    pub inner: Application<Id, Message, UserEvent>,
    pub screen: Screen,
    pub tx: mpsc::Sender<UserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge<T>,
}

impl<T> App<T>
where
    T: TerminalAdapter,
{
    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                self.inner.view(&Id::NameInput, f, f.area());
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
            Message::Tick => None,
            Message::ChangeScreen(screen) => {
                let _ = self.tx.send(UserEvent::ChangeScreen(screen));
                None
            }
            Message::None => None,
        }
    }
}

impl Default for App<CrosstermTerminalAdapter> {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        let mut app = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .add_port(Box::new(UserEventPort { rx }), Duration::from_millis(20), 1)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        app.mount(
            Id::Timer,
            Box::new(Timer::new(Duration::from_millis(CONFIG.game_duration))),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )
        .unwrap();

        app.mount(Id::NameInput, Box::new(NameInput::default()), Vec::new())
            .unwrap();

        app.mount(
            Id::QuitListener,
            Box::new(QuitListener::default()),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )
        .unwrap();

        app.active(&Id::NameInput).unwrap();

        Self {
            inner: app,
            screen: Screen::Home,
            tx,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().unwrap(),
        }
    }
}
