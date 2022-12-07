#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum DisplayState {
    SimulateScreen,
    LoadingScreen,
    Menu,
}
