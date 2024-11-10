pub struct App {
    pub exit: bool,
    pub state: AppState,
}

#[derive(PartialEq)]
pub enum AppState {
    Menu,
    InGame,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            state: AppState::InGame,
        }
    }
}
