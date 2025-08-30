use rusqlite::types::Value;
use tui_realm_stdlib::Table;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Style, TextSpan},
};

use crate::{app::Message, util::query::Row};

fn into_text_span(value: Value) -> TextSpan {
    match value {
        Value::Null => TextSpan::new("Null"),
        Value::Integer(n) => TextSpan::new(n.to_string()),
        Value::Real(n) => TextSpan::new(n.to_string()),
        Value::Text(s) => TextSpan::new(s),
        Value::Blob(items) => {
            TextSpan::new(items.into_iter().map(|x| x.to_string()).collect::<String>())
        }
    }
}

#[derive(MockComponent)]
pub struct ResultTable {
    component: Table,
}

impl ResultTable {
    pub fn new(headers: Vec<String>, rows: Vec<Row>) -> Self {
        let rows = rows
            .into_iter()
            .map(|row| row.into_iter().map(into_text_span).collect())
            .collect();

        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .sides(BorderSides::all())
                        .color(Color::Green),
                )
                .inactive(Style::reset())
                .title("Score", Alignment::Center)
                .scroll(true)
                .step(5)
                .row_height(1)
                .headers(headers)
                .table(rows),
        }
    }
}

impl Component<Message, NoUserEvent> for ResultTable {
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
