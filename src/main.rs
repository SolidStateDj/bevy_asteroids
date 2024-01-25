// mod input;
mod player;
mod asteroids;
mod collisions;
mod schedules;
mod despawn;
mod movement;
mod asset_loader;
mod state;
mod menu;
// mod bullets;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::{prelude::*, window::PrimaryWindow};
use collisions::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use menu::MenuPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use state::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_state::<AppState>()

        // Custom Plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(MenuPlugin)

        .add_systems(Startup, setup)

        .run();
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

