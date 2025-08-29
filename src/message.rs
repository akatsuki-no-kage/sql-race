#[derive(Debug, PartialEq)]
pub enum Message {
    AppClose,
    Tick,
    Start,
    End,
    None,
}
