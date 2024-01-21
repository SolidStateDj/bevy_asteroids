use bevy::prelude::*;

use crate::{collisions::Collider, schedules::InGameSet};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub sprite: SpriteBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position).chain().in_set(InGameSet::EntityUpdates)); 
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&mut Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (mut acceleration, mut velocity) in query.iter_mut() {
        // acceleration.value = Vec3::new(0.1, 0.0, 0.0);
        velocity.value += acceleration.value;
        // println!("{}", acceleration.value);
        // acceleration.value = Vec3::ZERO;
    }
}

