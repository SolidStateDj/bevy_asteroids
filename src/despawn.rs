use bevy::prelude::*;

use crate::schedules::InGameSet;

const DESPAWN_DISTANCE: f32 = 750.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_entities.in_set(InGameSet::DespawnEntities),);
    }
}

fn despawn_far_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::new(1280.0 / 2.0, 720.0 / 2.0, 0.0));
        // println!("Distance: {}", distance);

        if distance > DESPAWN_DISTANCE {
            println!("Despawned entity");
            commands.entity(entity).despawn_recursive();
        }
    }
}

