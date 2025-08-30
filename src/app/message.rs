use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Close,
    Start(String),
    ChangeScreen(Screen),
    ActiveNext,
    None,
}
