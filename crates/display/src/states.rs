use bevy::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Copy, States)]
pub enum DisplayState {
    SimulateScreen,
    #[default]
    LoadingScreen,
    Menu,
    Blueprint,
}
