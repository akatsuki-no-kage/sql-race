use ratatui::Frame;
use tuirealm::{Application, NoUserEvent};

use crate::app::{Id, Message, Screen};

fn draw_home(app: &mut Application<Id, Message, NoUserEvent>, f: &mut Frame) {
    app.view(&Id::UsernameInput, f, f.area());
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
