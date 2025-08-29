use tui_realm_stdlib::Phantom;
use tuirealm::{
    Component, Event,
    event::{Key, KeyEvent, KeyModifiers},
};
use tuirealm_derive::MockComponent;

use crate::app::{Message, UserEvent};

#[derive(MockComponent, Default)]
pub struct QuitListener {
    component: Phantom,
}

impl Component<Message, UserEvent> for QuitListener {
    fn on(&mut self, event: Event<UserEvent>) -> Option<Message> {
        match event {
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Message::Close),
            _ => None,
        }
    }
}
