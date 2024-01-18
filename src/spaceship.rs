use bevy::prelude::*;

use crate::{movement::{Velocity, MovingObjectBundle, Acceleration}, asset_loader::SceneAssets, collision::Collider, schedules::InGameSet};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_MOVEMENT_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, (
                    spaceship_movement_controls, 
                    spaceship_weapon_controls,
                    spaceship_shield_controls,
                ).chain().in_set(InGameSet::UserInput),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(
        (MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle { 
                scene: scene_assets.spaceship.clone(), 
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        }, 
        Spaceship,
    ));
}

fn spaceship_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>,) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // Rotation
    if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    // Movement
    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_MOVEMENT_SPEED;
    }

    // Roll
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // Rotate around the Y-Axis
    transform.rotate_y(rotation);

    // Rotate around the local Z-Axis
    transform.rotate_local_z(roll);

    // update the spaceship velocity based on the new direction
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(mut commands: Commands, query: Query<&Transform, With<Spaceship>>, keyboard_input: Res<Input<KeyCode>>, scene_assets: Res<SceneAssets>) {
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}

fn spaceship_shield_controls(mut commands: Commands, query: Query<Entity, With<Spaceship>>, keyboard_input: Res<Input<KeyCode>>) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}