use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Close,
    Play(String),
    ChangeScreen(Screen),
    FocusNext,
    None,
}
