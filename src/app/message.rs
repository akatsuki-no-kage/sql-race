use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Close,
    Tick,
    ChangeScreen(Screen),
    None,
}
