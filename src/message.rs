#[derive(Debug, PartialEq)]
pub enum Message {
    Close,
    Tick,
    Start,
    End,
    None,
}
