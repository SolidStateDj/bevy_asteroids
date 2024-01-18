use bevy::{prelude::*, window::PrimaryWindow};

pub const ASTEROID_SPAWNRATE: f32 = 5.0;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
/*         app.add_systems(Startup, (
            
        )); */
        app.add_systems(Update, (
            spawn_asteroids,
            tick_asteroid_spawn_timer,
        ));
    }
}

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
enum AsteroidSize {
    BIG,
    MED,
    SML,
}

struct AsteroidSizeData {
    big: SpriteBundle,
    med: SpriteBundle,
    sml: SpriteBundle
}

#[derive(Bundle)]
struct AsteroidBundle {
    size: AsteroidSize,

}

fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    asteroids_spawn_timer: Res<AsteroidSpawnTimer>,
) {
    if asteroids_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        
        // Pick random coords


    } 
}


// Asteroid Timer
#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> Self {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWNRATE, TimerMode::Repeating),
        }
    } 
}

fn tick_asteroid_spawn_timer(mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>, time: Res<Time>) {
    asteroid_spawn_timer.timer.tick(time.delta());
}


