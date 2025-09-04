use inkjet::{
    Language,
    theme::{Theme, vendored},
};
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
            component: TextArea::new(
                Vec::new(),
                Language::Sql,
                Theme::from_helix(vendored::DARK_PLUS).unwrap(),
            )
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
        let cmd = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => Cmd::Delete,

            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => Cmd::Cancel,

            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::SHIFT,
            }) => Cmd::Scroll(Direction::Down),

            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::SHIFT,
            }) => Cmd::Scroll(Direction::Up),

            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),

            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::SHIFT,
            }) => textarea::command::MOVE_WORD_BACK,

            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Cmd::Move(Direction::Left),

            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::SHIFT,
            }) => textarea::command::MOVE_WORD_FORWARD,

            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Cmd::Move(Direction::Right),

            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),

            Event::Keyboard(KeyEvent { code: Key::End, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            }) => Cmd::GoTo(Position::End),

            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::CONTROL,
            }) => textarea::command::NEWLINE,

            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => Cmd::GoTo(Position::Begin),

            Event::Keyboard(KeyEvent {
                code: Key::Char('v'),
                modifiers: KeyModifiers::CONTROL,
            }) => textarea::command::PASTE,

            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            }) => textarea::command::UNDO,

            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::CONTROL,
            }) => textarea::command::REDO,

            Event::Keyboard(KeyEvent {
                code: Key::Char('r') | Key::Char('s') | Key::Char('t'),
                modifiers: KeyModifiers::CONTROL,
            }) => Cmd::None,

            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => Cmd::Type(ch),

            _ => Cmd::None,
        };

        self.perform(cmd);

        match cmd {
            Cmd::None => None,
            _ => Some(Message::None),
        }
    }
}
