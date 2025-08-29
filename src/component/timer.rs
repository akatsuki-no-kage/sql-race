use std::time::Duration;

use ratatui::{Frame, layout::Rect};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, State, StateValue,
    command::{Cmd, CmdResult},
    props::{Alignment, BorderSides, Borders},
};

use crate::{
    app::{Message, Screen, UserEvent},
    config::CONFIG,
};

use super::text::Text;

pub struct TimerStates {
    duration: Duration,
    current: Duration,
    is_stopped: bool,
}

impl TimerStates {
    fn new(duration: Duration) -> Self {
        Self {
            duration,
            current: duration,
            is_stopped: true,
        }
    }

    fn start(&mut self) {
        self.is_stopped = false;
    }

    fn tick(&mut self) {
        if self.is_stopped {
            return;
        }

        self.current = self
            .current
            .saturating_sub(Duration::from_millis(CONFIG.tick_rate));
    }

    fn get_time_left(&self) -> u64 {
        self.current.as_secs()
    }

    fn reset(&mut self) {
        self.current = self.duration;
        self.is_stopped = true;
    }
}

pub struct Timer {
    component: Text,
    pub states: TimerStates,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            component: Text::default(),
            states: TimerStates::new(duration),
        }
    }

    pub fn title(mut self, title: impl AsRef<str>, alignment: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((title.as_ref().to_string(), alignment)),
        );
        self
    }

    pub fn border(mut self, side: BorderSides) -> Self {
        self.attr(
            Attribute::Borders,
            AttrValue::Borders(Borders::default().sides(side)),
        );
        self
    }
}

impl MockComponent for Timer {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area)
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value)
    }

    fn state(&self) -> State {
        State::One(StateValue::U64(self.states.get_time_left()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<Message, UserEvent> for Timer {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::Tick => {
                self.states.tick();

                let time_left = self.states.get_time_left();

                self.attr(Attribute::Text, AttrValue::String(time_left.to_string()));

                if time_left == 0 {
                    self.states.reset();
                    Some(Message::ChangeScreen(Screen::Home))
                } else {
                    Some(Message::Tick)
                }
            }
            Event::User(UserEvent::ChangeScreen(Screen::Game)) => {
                self.states.start();
                None
            }
            _ => None,
        }
    }
}
