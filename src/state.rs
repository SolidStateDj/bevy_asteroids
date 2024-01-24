use bevy::prelude::*;
pub struct AppState;

impl Plugin for AppState {
     fn build(&self, app: &mut App) {
         
     }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimState {
    #[default]
    InGame,
    Paused,
}