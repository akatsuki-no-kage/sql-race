use tui_realm_stdlib::Table;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Style, TextSpan},
};

use crate::{app::Message, config::CONFIG, repository::ScoreRepository};

#[derive(MockComponent)]
pub struct ScoreTable {
    component: Table,
}

impl Default for ScoreTable {
    fn default() -> Self {
        let repository = ScoreRepository::new(&CONFIG.database_file).unwrap();
        let scores = repository
            .get_all()
            .unwrap()
            .into_iter()
            .map(|score| {
                vec![
                    TextSpan::from(score.username),
                    TextSpan::from(score.score.to_string()),
                    TextSpan::from(score.created_at.to_string()),
                ]
            })
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
                .headers(["Username", "Score", "Time"])
                .table(scores),
        }
    }
}

impl Component<Message, NoUserEvent> for ScoreTable {
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
