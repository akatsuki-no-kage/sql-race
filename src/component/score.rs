use ratatui::layout::{Constraint, Direction};
use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    props::{BorderSides, Borders, Layout},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct Score {
    component: Container,
}

impl Score {
    pub fn new(score: u64) -> Self {
        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .layout(
                    Layout::default()
                        .constraints(&[Constraint::Min(0)])
                        .direction(Direction::Horizontal)
                        .margin(1),
                )
                .children(vec![Box::new(Label::default().text(score.to_string()))]),
        }
    }
}

impl Component<Message, NoUserEvent> for Score {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
