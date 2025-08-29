pub mod app;
pub mod component;
pub mod config;
pub mod repository;

use tuirealm::{PollStrategy, Update};

use crate::app::App;

fn main() {
    let mut app = App::default();

    app.terminal.enter_alternate_screen().unwrap();
    app.terminal.enable_raw_mode().unwrap();

    while !app.quit {
        match app.inner.tick(PollStrategy::Once) {
            Ok(messages) if !messages.is_empty() => {
                app.redraw = true;
                for message in messages {
                    let mut message = Some(message);
                    while message.is_some() {
                        message = app.update(message);
                    }
                }
            }
            Err(_) => {}
            _ => {}
        }

        if app.redraw {
            app.view();
            app.redraw = false;
        }
    }

    app.terminal.leave_alternate_screen().unwrap();
    app.terminal.disable_raw_mode().unwrap();
    app.terminal.clear_screen().unwrap();
}
