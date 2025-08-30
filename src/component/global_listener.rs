use tui_realm_stdlib::Phantom;
use tuirealm::{
    Component, Event, NoUserEvent,
    event::{Key, KeyEvent, KeyModifiers},
};
use tuirealm_derive::MockComponent;

use crate::app::Message;

#[derive(MockComponent, Default)]
pub struct GlobalListener {
    component: Phantom,
}

impl Component<Message, NoUserEvent> for GlobalListener {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        match event {
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Message::End),

            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => Some(Message::ActiveNext),
            _ => None,
        }
    }
}
