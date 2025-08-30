use crate::app::Screen;

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    ToggleHelp,
    Start(String),
    NextQuestion,
    End,
    ChangeScreen(Screen),
    Active(isize),
    None,
}
