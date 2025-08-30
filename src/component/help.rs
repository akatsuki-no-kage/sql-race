use tui_realm_stdlib::Table;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Style, TableBuilder, TextSpan},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct Help {
    component: Table,
}

impl Default for Help {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .sides(BorderSides::all())
                        .color(Color::Green),
                )
                .inactive(Style::reset())
                .title("Help", Alignment::Center)
                .scroll(true)
                .step(5)
                .highlighted_color(Color::Cyan)
                .row_height(1)
                .headers(["Key", "Description"])
                .table(
                    TableBuilder::default()
                        .add_col(TextSpan::new("Ctrl + H"))
                        .add_col(TextSpan::new("Show help"))
                        .add_row()
                        .add_col(TextSpan::new("F1"))
                        .add_col(TextSpan::new("Show help"))
                        .add_row()
                        .add_row()
                        .add_col(TextSpan::new("Tab"))
                        .add_col(TextSpan::new("Focus next component"))
                        .add_row()
                        .add_col(TextSpan::new("Shift + Tab"))
                        .add_col(TextSpan::new("Focus previous component"))
                        .add_row()
                        .add_row()
                        .add_col(TextSpan::new("Ctrl + s"))
                        .add_col(TextSpan::new("Show schema"))
                        .add_row()
                        .add_col(TextSpan::new("Ctrl + r"))
                        .add_col(TextSpan::new("Run current query"))
                        .add_row()
                        .add_col(TextSpan::new("Ctrl + s"))
                        .add_col(TextSpan::new("Submit current query"))
                        .add_row()
                        .build(),
                ),
        }
    }
}

impl Component<Message, NoUserEvent> for Help {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        let _ = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            _ => CmdResult::None,
        };
        Some(Message::None)
    }
}
