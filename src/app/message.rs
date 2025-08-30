use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Start(String),
    NextQuestion,
    End,
    ChangeScreen(Screen),
    Active(isize),
    None,
}
