#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Screen {
    #[default]
    Home,
    Game,
}

#[derive(Default)]
pub struct AppState {
    pub name: Option<String>,
    pub screen: Screen,
}
