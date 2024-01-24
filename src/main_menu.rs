use bevy::{prelude::*};

use crate::state::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
     fn build(&self, app: &mut App) {
         app
          .add_systems(OnEnter(AppState::Menu), setup_menu)
          .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
          ;
     }
}

fn setup_menu(mut commands: Commands) {

}

fn menu(mut next_state: ResMut<NextState<AppState>>) {

}