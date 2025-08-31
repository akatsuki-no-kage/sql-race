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
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Message::Quit),

            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Message::Active(1)),

            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => Some(Message::Active(-1)),

            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Message::ToggleHelp),

            Event::Keyboard(KeyEvent {
                code: Key::Function(1),
                ..
            }) => Some(Message::ToggleHelp),

            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Message::Run),

            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Message::Submit),

            _ => None,
        }
    }
}
