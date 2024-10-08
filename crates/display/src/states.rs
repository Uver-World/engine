use bevy::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Copy, States)]
pub enum DisplayState {
    #[default]
    Setup,
    SimulateScreen,
    LoadingScreen,
    Menu,
    Blueprint,
}
