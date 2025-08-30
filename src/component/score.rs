use tui_realm_stdlib::Paragraph;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    props::{Alignment, BorderSides, Borders, TextSpan},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct Score {
    component: Paragraph,
}

impl Score {
    pub fn new(score: u64) -> Self {
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .title("Score", Alignment::Center)
                .text([TextSpan::new(score.to_string())])
                .alignment(Alignment::Center),
        }
    }
}

impl Component<Message, NoUserEvent> for Score {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
