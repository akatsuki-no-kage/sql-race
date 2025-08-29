use tui_realm_stdlib::Table;
use tuirealm::{
    Component, NoUserEvent,
    props::{Alignment, BorderSides, Borders, TableBuilder, TextSpan},
};
use tuirealm_derive::MockComponent;

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
                .borders(Borders::default().sides(BorderSides::all()))
                .title("Score", Alignment::Center)
                .scroll(true)
                .rewind(true)
                .step(5)
                .row_height(1)
                .headers(["Username", "Score", "Time"])
                .table(scores),
        }
    }
}

impl Component<Message, NoUserEvent> for ScoreTable {
    fn on(&mut self, _: tuirealm::Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
