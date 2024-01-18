use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Clone, SystemSet, Eq)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            InGameSet::DespawnEntities,
            // Flush Commands
            InGameSet::UserInput,
            InGameSet::EntityUpdates,
            InGameSet::CollisionDetection,
        ).chain(),
        ).add_systems(Update, apply_deferred
            .after(InGameSet::DespawnEntities)
            .before(InGameSet::UserInput),
        );
        
    }
}

