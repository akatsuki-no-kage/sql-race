use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, Borders, Color, Style, TextModifiers},
};

use crate::{
    app::Message,
    component::textarea::{self, TextArea},
};

#[derive(MockComponent)]
pub struct Editor<'a> {
    component: TextArea<'a>,
}

impl Default for Editor<'_> {
    fn default() -> Self {
        Self {
            component: TextArea::default()
                .borders(
                    Borders::default()
                        .color(Color::Green)
                        .sides(BorderSides::all()),
                )
                .inactive(Style::reset())
                .cursor_line_style(Style::default())
                .cursor_style(Style::default().add_modifier(TextModifiers::REVERSED))
                .line_number_style(
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(TextModifiers::ITALIC),
                )
                .max_histories(64)
                .scroll_step(4)
                .tab_length(4)
                .title("Editor", Alignment::Center),
        }
    }
}

impl Component<Message, NoUserEvent> for Editor<'_> {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        match event {
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Delete);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Message::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
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
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up));
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
                code: Key::Enter, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(textarea::command::NEWLINE);
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
