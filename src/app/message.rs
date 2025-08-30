use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Help,
    Start(String),
    NextQuestion,
    End,
    ChangeScreen(Screen),
    Active(isize),
    None,
}
