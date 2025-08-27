use std::time::Duration;

use ratatui::{Frame, layout::Rect};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, State, StateValue,
    command::{Cmd, CmdResult},
    props::{Alignment, BorderSides, Borders},
};

use crate::{Message, config::CONFIG, event::UserEvent};

use super::text::Text;

struct OwnState {
    duration: Duration,
    current: Duration,
    is_stopped: bool,
}

impl OwnState {
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
    state: OwnState,
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
        State::One(StateValue::U64(self.state.get_time_left()))
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, UserEvent> for Timer {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::Tick => {
                self.state.tick();
                self.attr(
                    Attribute::Text,
                    AttrValue::String(self.state.get_time_left().to_string()),
                );
                if self.state.get_time_left() == 0 {
                    self.state.reset();
                    Some(Message::End)
                } else {
                    Some(Message::Tick)
                }
            }
            Event::User(UserEvent::Start) => {
                self.state.start();
                None
            }
            _ => None,
        }
    }
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            component: Text::default(),
            state: OwnState::new(duration),
        }
    }

    pub fn title(mut self, title: Option<String>, alignment: Option<Alignment>) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((
                title.unwrap_or_default(),
                alignment.unwrap_or(Alignment::Left),
            )),
        );
        self
    }

    pub fn border_side(mut self, side: BorderSides) -> Self {
        self.attr(
            Attribute::Borders,
            AttrValue::Borders(Borders::default().sides(side)),
        );
        self
    }
}
