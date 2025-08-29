use ratatui::style::Style;
use tuirealm::{
    Component, Event, MockComponent,
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, Borders},
};

use crate::{
    Message,
    component::textarea::{self, TextArea},
    event::UserEvent,
};

#[derive(MockComponent)]
pub struct NameInput<'a> {
    component: TextArea<'a>,
}

impl Default for NameInput<'_> {
    fn default() -> Self {
        Self {
            component: TextArea::default()
                .single_line(true)
                .borders(Borders::default().sides(BorderSides::all()))
                .cursor_line_style(Style::default())
                .title("Name", Alignment::Center),
        }
    }
}

impl Component<Message, UserEvent> for NameInput<'_> {
    fn on(&mut self, event: Event<UserEvent>) -> Option<Message> {
        match event {
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(textarea::command::MOVE_WORD_BACK);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(textarea::command::MOVE_WORD_FORWARD);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('v'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(textarea::command::PASTE);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(textarea::command::UNDO);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(textarea::command::REDO);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                self.perform(Cmd::Type('\t'));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Message::None)
            }
            _ => None,
        }
    }
}
