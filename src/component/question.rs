use ratatui::layout::{Constraint, Direction};
use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    props::{Alignment, BorderSides, Borders, Layout},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct Question {
    component: Container,
}

impl Question {
    pub fn new(question: String) -> Self {
        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .title("Question", Alignment::Center)
                .layout(
                    Layout::default()
                        .constraints(&[Constraint::Min(0)])
                        .direction(Direction::Horizontal)
                        .margin(1),
                )
                .children(vec![Box::new(Label::default().text(question))]),
        }
    }
}

impl Component<Message, NoUserEvent> for Question {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
