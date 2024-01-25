use bevy::{prelude::*, window::PrimaryWindow};

use crate::{schedules::InGameSet, state::AppState, MainCamera};

const DESPAWN_DISTANCE: f32 = 750.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_entities.run_if(in_state(AppState::InGame)).in_set(InGameSet::DespawnEntities),);
    }
}

fn despawn_far_entities(
    mut commands: Commands, 
    query: Query<(Entity, &GlobalTransform), Without<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,

) {
    let window = window_query.get_single().unwrap();
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0));
        // println!("Distance: {}", distance);

        if distance > DESPAWN_DISTANCE {
            println!("Despawned entity");
            commands.entity(entity).despawn_recursive();
        }
    }
}

