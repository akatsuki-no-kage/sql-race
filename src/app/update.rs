use tuirealm::{Update, terminal::TerminalAdapter};

use super::{App, Message, Screen};

impl<T> Update<Message> for App<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, message: Option<Message>) -> Option<Message> {
        let message = message?;
        self.redraw = true;

        match message {
            Message::Close => {
                self.quit = true;

                None
            }

            Message::Start(username) => {
                self.username = Some(username);
                self.question_index = 0;

                Some(Message::ChangeScreen(Screen::Game))
            }

            Message::NextQuestion => {
                self.question_index += 1;

                if self.question_index == self.questions.len() {
                    Some(Message::End)
                } else {
                    None
                }
            }

            Message::End => {
                self.username = None;

                Some(Message::ChangeScreen(Screen::Home))
            }

            Message::ChangeScreen(screen) => {
                self.screen = screen;
                self.mount_all();

                None
            }

            Message::ActiveNext => {
                self.active_next().unwrap();

                None
            }

            Message::None => None,
        }
    }
}
