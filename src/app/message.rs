use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Close,
    Start(String),
    NextQuestion,
    End,
    ChangeScreen(Screen),
    ActiveNext,
    None,
}
