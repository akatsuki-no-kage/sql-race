use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Close,
    Start(String),
    End,
    ChangeScreen(Screen),
    ActiveNext,
    None,
}
