use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, State, StateValue,
    command::{Cmd, CmdResult},
    props::{BorderSides, Borders, Layout},
};

use crate::app::Message;

pub struct OwnStates {
    time_left: Duration,
    tick_rate: Duration,
    is_disable: bool,
}

impl OwnStates {
    fn new(duration: Duration, tick_rate: Duration) -> Self {
        Self {
            time_left: duration,
            tick_rate,
            is_disable: false,
        }
    }

    fn tick(&mut self) {
        if self.is_disable {
            return;
        }

        self.time_left = self.time_left.saturating_sub(self.tick_rate);
    }

    fn get_second_left(&self) -> u64 {
        self.time_left.as_secs()
    }
}

pub struct Timer {
    component: Container,
    pub states: OwnStates,
}

impl Timer {
    pub fn new(duration: Duration, tick_rate: Duration) -> Self {
        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .layout(
                    Layout::default()
                        .constraints(&[Constraint::Min(0)])
                        .direction(Direction::Horizontal)
                        .margin(1),
                )
                .children(vec![Box::new(Label::default())]),
            states: OwnStates::new(duration, tick_rate),
        }
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
        State::One(StateValue::U64(self.states.get_second_left()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<Message, NoUserEvent> for Timer {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        match event {
            Event::Tick => {
                self.states.tick();

                let second_left = self.states.get_second_left();

                self.attr(Attribute::Text, AttrValue::String(second_left.to_string()));

                if second_left > 0 {
                    return Some(Message::None);
                }

                if self.states.is_disable {
                    None
                } else {
                    Some(Message::End)
                }
            }
            _ => None,
        }
    }
}
