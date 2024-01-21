use bevy::{prelude::*, window::PrimaryWindow, transform};

pub const PLAYER_SIZE: f32 = 0.5;
pub const HALF_PLAYER_SIZE: f32 = 16.0;
pub const PLAYER_TIME_UNTIL_NEXT_SHOT: f32 = 0.15;

pub const MISSILE_SPEED: f32 = 500.0;
pub const MISSILE_SIZE: f32 = 0.1;

use crate::{bullets::{Bullet, BulletsPlugin}, schedules::InGameSet, movement::{MovingObjectBundle, Velocity, Acceleration, self}, collisions::Collider, asset_loader::SceneAssets, player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                BulletsPlugin,
        ));
        app.init_resource::<PlayerFirerateTimer>();
        app.add_systems(PostStartup, spawn_player);
        app.add_systems(Update, (
            player_movement,
            confine_player_movement,
            player_weapon,
            player_shield,
        ).chain().in_set(InGameSet::UserInput));
        app.add_systems(Update, (
            tick_player_shot_timer,
        ));
    } 
}


// Player Stuff
// A player marker
#[derive(Component)]
pub struct Player {
    player_data: PlayerData,
}

#[derive(Component, Debug)]
pub struct PlayerBullet;

#[derive(Component, Debug)]
pub struct PlayerShield;

// Player Data
#[derive(Component)]
struct PlayerData {
    lives: u32,
    rpm: f32,
    pub can_fire: bool,
    pub boosting: Vec3,
    pub speed: f32,
    pub max_speed: f32,
    pub rotation_speed: f32,
    pub firerate: f32,
    stats: Stats,
}
// Stats for the player
struct Stats {
    score: u32,
    asteroids_destroyed: u32,
    level: u32,
    shots_fired: u32,
}

// Spawns the player bundle
fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, scene_assets: Res<SceneAssets>,) {
    let window = window_query.get_single().unwrap();
    let player: Handle<Image> = scene_assets.spaceship.clone();

    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        collider: Collider::new(PLAYER_SIZE),
        sprite: SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0).with_scale(Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 0.0)),
            texture: player, 
            ..default()
        }
    }, Player {
        player_data: PlayerData { 
            lives: 3, 
            rpm: 60.0,
            can_fire: true,
            boosting: Vec3::ZERO,
            speed: 200.0,
            max_speed: 10.0,
            rotation_speed: 4.0,
            firerate: PLAYER_TIME_UNTIL_NEXT_SHOT,
            stats: Stats { 
                score: 0, 
                asteroids_destroyed: 0, 
                level: 1, 
                shots_fired: 0 
            },
        },
    }));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    player_data: Query<&Player>,
    mut player_query: Query<(&mut Transform, &mut Acceleration), With<Player>>, 
    time: Res<Time>,
) {
    let Ok(player) = player_data.get_single() else {
        println!("Brokey");
        return;
    };
    let Ok((mut transform, mut acceleration)) = player_query.get_single_mut() else {
        return;
    };

    let mut rotation = 0.0;
    let mut movement = 0.0;
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        movement = player.player_data.speed;
        direction += transform.up();
    }
    // Rotate Left
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        rotation = player.player_data.rotation_speed * time.delta_seconds();
    }
    // Slow Down
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        movement = -player.player_data.speed;
        direction += transform.up();
    }
    // Rotate Right
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        rotation = -player.player_data.rotation_speed * time.delta_seconds();
    }
    transform.rotate_z(rotation);

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    // println!("{}", direction);
    acceleration.value = movement * direction * time.delta_seconds(); 
    // transform.translation += direction * player.player_data.speed * time.delta_seconds();
    // println!("{}", acceleration.value);
}

fn player_weapon(
    mut commands: Commands, 
    transform_query: Query<&Transform, With<Player>>, 
    mut player_data: Query<&mut Player>,
    timer: Res<PlayerFirerateTimer>, 
    keyboard_input: Res<Input<KeyCode>>, 
    mouse_input: Res<Input<MouseButton>>,
    scene_assets: Res<SceneAssets>
) {
    let transform = transform_query.single();
    let bullet: Handle<Image> = scene_assets.bullet.clone();
    let Ok(mut player) = player_data.get_single_mut() else {
        println!("Couldn't Get Player");
        return; 
    };

    if keyboard_input.pressed(KeyCode::Space) || mouse_input.pressed(MouseButton::Left) {
        if player.player_data.can_fire {
            commands.spawn((MovingObjectBundle {
                velocity: Velocity::new(transform.up() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::new(0.0, 0.0, 0.0)),
                collider: Collider::new(MISSILE_SIZE),
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z) + (15.0 * transform.up()),
                        rotation: transform.rotation,
                        ..default()
                    },
                    texture: bullet,
                    ..default()
                }, 
            }, PlayerBullet,));
            player.player_data.stats.shots_fired += 1;
            player.player_data.can_fire = false;
        } else if timer.timer.finished() {
            player.player_data.can_fire = true;
        }
    } 

}

fn player_shield(mut commands: Commands, query: Query<Entity, With<Player>>, keyboard_input: Res<Input<KeyCode>>) {
    let Ok(player) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(player).insert(PlayerShield);
    }
}

fn confine_player_movement(mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok((mut player_transform, mut player_velocity)) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        
        let x_min = 0.0 + HALF_PLAYER_SIZE;
        let x_max = window.width() - HALF_PLAYER_SIZE;
        let y_min = 0.0 + HALF_PLAYER_SIZE;
        let y_max = window.height() - HALF_PLAYER_SIZE;


        let mut translation = player_transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
            player_velocity.value *= Vec3::new(0.0, 1.0, 0.0);
        } else if translation.x > x_max {
            translation.x = x_max;
            player_velocity.value *= Vec3::new(0.0, 1.0, 0.0);
        }
        if translation.y < y_min {
            translation.y = y_min;
            player_velocity.value *= Vec3::new(1.0, 0.0, 0.0);
        } else if translation.y > y_max {
            translation.y = y_max;
            player_velocity.value *= Vec3::new(1.0, 0.0, 0.0);
        }

        player_transform.translation = translation;
    }
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

// Timer until player can fire another bullet.
fn tick_player_shot_timer(mut bullet_firerate_timer: ResMut<PlayerFirerateTimer>, time: Res<Time>) {
    bullet_firerate_timer.timer.tick(time.delta());
}
