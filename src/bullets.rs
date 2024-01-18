use bevy::{prelude::*, window::PrimaryWindow};

pub const BULLET_SPEED: f32 = 250.0;
pub const BULLET_DESPAWN_DISTANCE: f32 = 50.0;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_bullets,
            despawn_bullets,
        ));
    }
}

// Bullet Stuff
#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        let direction = Vec3::new(1.0, 1.0, 0.0);
        transform.translation += direction * BULLET_SPEED * time.delta_seconds();
    }
}

fn despawn_bullets(mut commands: Commands, bullet_query: Query<((Entity, With<Bullet>), &Transform)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let x_min = 0.0 - BULLET_DESPAWN_DISTANCE;
    let x_max = window.width() + BULLET_DESPAWN_DISTANCE;
    let y_min = 0.0 - BULLET_DESPAWN_DISTANCE;
    let y_max = window.height() + BULLET_DESPAWN_DISTANCE;

    for (bullet, transform) in bullet_query.iter() {    
        if transform.translation.x < x_min {
            println!("Despawned at {}", transform.translation.xy());
            commands.entity(bullet.0).despawn_recursive();
        } else if transform.translation.x > x_max {
            println!("Despawned at {}", transform.translation.xy());
            commands.entity(bullet.0).despawn_recursive();
        }
        if transform.translation.y < y_min {
            println!("Despawned at {}", transform.translation.xy());
            commands.entity(bullet.0).despawn_recursive();
        } else if transform.translation.y > y_max {
            println!("Despawned at {}", transform.translation.xy());
            commands.entity(bullet.0).despawn_recursive();
        }
    }
}

