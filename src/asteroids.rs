use std::ops::Range;

use bevy::{prelude::*, window::PrimaryWindow, render::texture};
use rand::Rng;

use crate::{movement::{MovingObjectBundle, Acceleration, Velocity}, collisions::Collider, asset_loader::SceneAssets};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>()
        .add_systems(Update, (
            spawn_asteroids,
            tick_asteroid_spawn_timer,
        ));
    }
}

pub const ASTEROID_SPAWNRATE: f32 = 5.0;

#[derive(Component)]
pub struct Asteroid;

fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    scene_assets: Res<SceneAssets>,
    asteroids_spawn_timer: Res<AsteroidSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();
    if !asteroids_spawn_timer.timer.finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    // Pick random coords
    let translation = Vec3::new(rng.gen_range(0.0..window.width()), rng.gen_range(0.0..window.height()), 0.0);

    // Pick Random Size
    let asteroid_size = rng.gen_range(0..3);

    let asteroid = SpriteBundle {
        transform: Transform::from_translation(translation).with_scale(Vec3::new(
            0.25 * (asteroid_size + 1) as f32, 
            0.25 * (asteroid_size + 1) as f32, 
            0.25 * (asteroid_size + 1) as f32
        )),
        texture: scene_assets.asteroids[asteroid_size].clone(),
        ..default()
    };

    // Pick Random Speed
    let velocity = Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero() * rng.gen_range(100.0..1000.0);

    commands.spawn((MovingObjectBundle {
        acceleration: Acceleration::new(Vec3::ZERO),
        velocity: Velocity::new(velocity),
        collider: Collider::new(asteroid_size as f32),
        sprite: asteroid,
    }, Asteroid, ));
    println!("Spawned Asteroid with size {} at {} with speed {}", asteroid_size, translation, velocity); 

}


// Asteroid Timer
#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> Self {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWNRATE, TimerMode::Repeating),
        }
    }
}

fn tick_asteroid_spawn_timer(mut bullet_firerate_timer: ResMut<AsteroidSpawnTimer>, time: Res<Time>) {
    bullet_firerate_timer.timer.tick(time.delta());
}
