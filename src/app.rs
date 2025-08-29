use std::{sync::mpsc, time::Duration};

use tuirealm::{
    Application, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
};

use crate::{
    Id, Message,
    component::{name_input::NameInput, timer::Timer},
    config::CONFIG,
    event::{UserEvent, UserEventPort},
};

pub struct App<T>
where
    T: TerminalAdapter,
{
    pub inner: Application<Id, Message, UserEvent>,
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
            Message::AppClose => {
                self.quit = true;
                None
            }
            Message::Tick => None,
            Message::Start => {
                let _ = self.tx.send(UserEvent::Start);
                None
            }
            Message::End => {
                let _ = self.tx.send(UserEvent::End);
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
            vec![Sub::new(
                SubEventClause::User(UserEvent::Start),
                SubClause::Always,
            )],
        )
        .unwrap();

        app.mount(Id::NameInput, Box::new(NameInput::default()), Vec::new())
            .unwrap();

        app.active(&Id::NameInput).unwrap();

        Self {
            inner: app,
            tx,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().unwrap(),
        }
    }
}
