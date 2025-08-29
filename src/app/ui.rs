use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};
use tuirealm::{Application, NoUserEvent};

use crate::app::{Id, Message, Screen};

fn draw_home(app: &mut Application<Id, Message, NoUserEvent>, f: &mut Frame) {
    let margined_chunks =
        Layout::horizontal([Constraint::Min(0), Constraint::Max(80), Constraint::Min(0)])
            .margin(2)
            .split(f.area());

    let chunks =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(margined_chunks[1]);

    app.view(&Id::ScoreTable, f, chunks[0]);
    app.view(&Id::UsernameInput, f, chunks[1]);
}

fn draw_game(app: &mut Application<Id, Message, NoUserEvent>, f: &mut Frame) {
    app.view(&Id::Timer, f, f.area());
}

pub fn draw(app: &mut Application<Id, Message, NoUserEvent>, screen: Screen, f: &mut Frame) {
    match screen {
        Screen::Home => draw_home(app, f),
        Screen::Game => draw_game(app, f),
    }
}
