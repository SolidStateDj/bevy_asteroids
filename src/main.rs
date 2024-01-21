// mod input;
mod player;
mod bullets;
mod asteroids;
mod collisions;
mod schedules;
mod despawn;
mod movement;
mod asset_loader;

use asset_loader::AssetLoaderPlugin;
use bevy::{prelude::*, window::PrimaryWindow};
use collisions::CollisionDetectionPlugin;
use leafwing_input_manager::{plugin::InputManagerPlugin, Actionlike};
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Custom Plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(CollisionDetectionPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Boost,
    Turn,
    Shoot,
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    }, MainCamera));
}


