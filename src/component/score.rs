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

impl Default for Score {
    fn default() -> Self {
        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .layout(
                    Layout::default()
                        .constraints(&[Constraint::Min(0)])
                        .direction(Direction::Horizontal)
                        .margin(1),
                )
                .children(vec![Box::new(Label::default().text("0"))]),
        }
    }
}

impl Component<Message, NoUserEvent> for Score {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
