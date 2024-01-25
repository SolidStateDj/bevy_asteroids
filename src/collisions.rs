use bevy::{prelude::*, utils::{HashMap}};

use crate::{schedules::InGameSet, asteroids::Asteroid, player::Player, player::PlayerBullet, state::AppState};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            collision_detection.run_if(in_state(AppState::InGame)).in_set(InGameSet::CollisionDetection),
        );
        app.add_systems(Update, (
            handle_asteroid_collisions,
            handle_player_collisions,
        ).run_if(in_state(AppState::InGame)).in_set(InGameSet::DespawnEntities),
        );
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // Get every combination of entity that has a collider
    let mut iter = query.iter_combinations_mut();     
    // For every combination
    while let Some([(entity_a, transform_a, collider_a),(entity_b, transform_b, collider_b)]) = iter.fetch_next() {
        // Get the distance between the two entities
        let distance = transform_a.translation().distance(transform_b.translation());
        // If their colliders intersect
        if distance < collider_a.radius + collider_b.radius {
            // Add entity_b to the vector of (key) entity_a in the newly created hashmap
            colliding_entities.entry(entity_a).or_insert_with(Vec::new).push(entity_b);
        }
    }

    // For every entity (with a collider)
    for (entity, _, mut collider) in query.iter_mut() {
        // Remove all existing localized collisions
        collider.colliding_entities.clear();
        // If the entity is colliding with anything (and therefore in colliding_entities)
        if let Some(collisions) = colliding_entities.get(&entity) {
            // Add the vector of colliding entities from colliding_entities to the localized
            // collisions of the entity
            collider.colliding_entities.extend(collisions.iter().copied());
        }
    }
}

fn handle_asteroid_collisions (
    mut commands: Commands, 
    asteroid_query: Query<(Entity, &Collider), With<Asteroid>>, 
    bullet_query: Query<&PlayerBullet>,
    mut player_query: Query<&mut Player>,
) {
    // For every asteroid 
    for (entity, collider) in asteroid_query.iter() {
        // For every entry in its local database of collisions
        for &collided_entity in collider.colliding_entities.iter() {
            // if the entity stored in the list of colliding_entities exists in the query for
            // colliders tagged with Asteroid it means an Asteroid has collided with an Asteroid.
            if asteroid_query.get(collided_entity).is_ok() {
                println!("Asteroid collided with asteroid");
                continue;
            }
            // An asteroid has collided with something that is not another asteroid

            // It never gets here whenever Menu is the initial state
            println!("It got here");

            let Ok(mut player) = player_query.get_single_mut() else { return; };
            if bullet_query.get(collided_entity).is_ok() {
                println!("Bullet and Asteroid Collision");
                player.player_data.stats.asteroids_destroyed += 1;
                commands.entity(entity).despawn_recursive();
                commands.entity(collided_entity).despawn_recursive();
            }

            // Player collision detection does not work in here for some reason
        }
    } 
}

fn handle_player_collisions (
    mut commands: Commands, 
    player_collider_query: Query<&Collider, With<Player>>, 
    asteroid_query: Query<&Asteroid>,
    mut player_query: Query<&mut Player>,
) {
    let collider = player_collider_query.get_single().unwrap();
    let Ok(mut player) = player_query.get_single_mut() else { return; };

    // For every entry in its local database of collisions
    for &collided_entity in collider.colliding_entities.iter() {
        if asteroid_query.get(collided_entity).is_ok() {
            println!("Player and Asteroid Collision");
            if player.player_data.lives - 1 > 0 {
                player.player_data.lives -= 1;
                player.player_data.stats.asteroids_destroyed += 1;
            }
            // commands.entity(entity).despawn_recursive();
            commands.entity(collided_entity).despawn_recursive();
        }
    }
}

