use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{axislike::DualAxisData, action_state::ActionState};

pub const PLAYER_SIZE: f32 = 0.5;
pub const HALF_PLAYER_SIZE: f32 = 16.0;
pub const PLAYER_SPEED: f32 = 2.0;
pub const PLAYER_TIME_UNTIL_NEXT_SHOT: f32 = 1./5.;

use crate::{PlayerAction, input::{ActiveInput, InputModeManagerPlugin}, bullets::{Bullet, BulletsPlugin}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                InputModeManagerPlugin, 
                BulletsPlugin,
        ));
        // Defined below, detects whether MKB or gamepad are active
        app.init_resource::<ActionState<PlayerAction>>();
        app.init_resource::<PlayerFirerateTimer>();
        app.insert_resource(PlayerAction::default_input_map());
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (
            player_mouse_look.run_if(in_state(ActiveInput::MouseKeyboard)),
            control_player.after(player_mouse_look),
            confine_player_movement,
            tick_player_shot_timer,
        ));
    } 
}

// A player marker
#[derive(Component)]
struct Player;

// Player Stuff
#[derive(Component)]
struct PlayerData {
    lives: u32,
    rpm: f32,
    pub can_fire: bool,
    stats: Stats,
    thrust: bool,
}
struct Stats {
    score: u32,
    asteroids_destroyed: u32,
    level: u32,
    shots_fired: u32,
}
#[derive(Bundle)]
struct PlayerBundle {
    player: PlayerData,
    sprite: SpriteBundle,
}


#[derive(Resource)]
pub struct PlayerFirerateTimer {
    pub timer: Timer,
}

impl Default for PlayerFirerateTimer {
    fn default() -> Self {
        PlayerFirerateTimer {
            timer: Timer::from_seconds(PLAYER_TIME_UNTIL_NEXT_SHOT, TimerMode::Repeating),
        }
    }
}

fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    let player: Handle<Image> = asset_server.load("sprites/Player.png");

    commands.spawn((PlayerBundle {
        player: PlayerData { 
            lives: 3, 
            rpm: 60.0,
            can_fire: true,
            stats: Stats { 
                score: 0, 
                asteroids_destroyed: 0, 
                level: 1, 
                shots_fired: 0 
            },
            thrust: false
        },
        sprite: SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0).with_scale(Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 0.0)),
            texture: player, 
            ..default()
        }
    }, Player));
}

fn player_mouse_look(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut action_state: ResMut<ActionState<PlayerAction>>,
) {
    if let Some(cursor_pos) = window_query.single().cursor_position() {
        action_state.press(PlayerAction::Look);
        let action_data = action_state.action_data_mut(PlayerAction::Look);
        action_data.axis_pair = Some(DualAxisData::from_xy(cursor_pos));
    }
}

fn control_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    action_state: Res<ActionState<PlayerAction>>,
    mut pl_query: Query<&mut PlayerData, With<Player>>,
    mut query: Query<&mut Transform, With<Player>>,
    timer: Res<PlayerFirerateTimer>,
) {
    let mut player_transform = query.single_mut();
    let mut player = pl_query.get_single_mut().unwrap();
    let bullet: Handle<Image> = asset_server.load("sprites/Square.png");
    if action_state.pressed(PlayerAction::Move) {
        let move_delta = action_state.clamped_axis_pair(PlayerAction::Move).unwrap().xy() * PLAYER_SPEED;
        player_transform.translation += Vec3::new(move_delta.x, move_delta.y, 0.0);
        println!("Player moved to: {}", player_transform.translation.xy());
    }

    if action_state.pressed(PlayerAction::Look) {
        let look = action_state.axis_pair(PlayerAction::Look).unwrap().xy();
        // println!("Player looking at point: {}", look);
        // let mut player_transform = query.get_single().expect("Need a single player");
        let player_pos: Vec2 = player_transform.translation.xy();
        let angle = (look - player_pos).angle_between(player_pos);
        player_transform.rotation = Quat::from_rotation_z(angle - FRAC_PI_2);
        // println!("Player is now at {} with rotation {}", player_transform.translation.xy(), player_transform.rotation);
    }

    if action_state.pressed(PlayerAction::Shoot) {
        if player.can_fire {
            println!("Shoot!");
            commands.spawn((SpriteBundle {
                texture: bullet,
                transform: Transform {
                    translation: Vec3::new(player_transform.translation.x, player_transform.translation.y, 0.0),
                    rotation: player_transform.rotation, 
                    // rotation: Quat::from_xyzw(player_transform.translation.x, player_transform.translation.y, player_transform.translation.z, 0.0),
                    ..default()
                },
                ..default()
            }, Bullet {
                direction: Vec2::new(player_transform.rotation.x, player_transform.rotation.y),
            }));
            player.can_fire = false;
        } else if timer.timer.finished() {
            println!("Timer Finished");
            player.can_fire = true;
        }
    }
}

fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        
        let x_min = 0.0 + HALF_PLAYER_SIZE;
        let x_max = window.width() - HALF_PLAYER_SIZE;
        let y_min = 0.0 + HALF_PLAYER_SIZE;
        let y_max = window.height() - HALF_PLAYER_SIZE;


        let mut translation = player_transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

fn tick_player_shot_timer(mut bullet_firerate_timer: ResMut<PlayerFirerateTimer>, time: Res<Time>) {
    bullet_firerate_timer.timer.tick(time.delta());
}
