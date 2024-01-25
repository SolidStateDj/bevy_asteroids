use bevy::{prelude::*, utils::{HashMap}};

use crate::{schedules::InGameSet, asteroids::Asteroid, player::Player, player::PlayerBullet, state::AppState, hud::{Lives, IMAGE_MARGIN, Score}, asset_loader::SceneAssets};

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

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>, bullets: Query<Entity, With<PlayerBullet>>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // Get every combination of entity that has a collider
    let mut iter = query.iter_combinations_mut();     
    // For every combination
    while let Some([(entity_a, transform_a, collider_a),(entity_b, transform_b, collider_b)]) = iter.fetch_next() {
        // Get the distance between the two entities
        let distance = transform_a.translation().xy().distance(transform_b.translation().xy());
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
    mut text_query: Query<&mut Text, With<Score>>,
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
                player.player_data.stats.score += 100;

                for mut text in text_query.iter_mut() {
                    text.sections[0].value = format!("Score: {}", player.player_data.stats.score);
                }
                
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
    mut lives_node: Query<Entity, With<Lives>>,
    scene_assets: Res<SceneAssets>,
) {
    let collider = player_collider_query.get_single().unwrap();
    let Ok(mut player) = player_query.get_single_mut() else { return; };
    let lives = lives_node.get_single_mut().unwrap();
    let life = UiImage::new(scene_assets.lives.clone());

    // For every entry in its local database of collisions
    for &collided_entity in collider.colliding_entities.iter() {
        if asteroid_query.get(collided_entity).is_ok() {
            println!("Player and Asteroid Collision");
            commands.entity(lives).despawn_recursive();
            if player.player_data.lives - 1 > 0 {
                player.player_data.lives -= 1;
                player.player_data.stats.asteroids_destroyed += 1;
                player.player_data.stats.score += 50;
            
                // THIS IS INCREDIBLE STUPID BUT I DO NOT HAVE THE PATIENCE TO FIGURE OUT HOW TO
                // PROPERLY DO IT AT THE MOMENT
                commands.entity(collided_entity).despawn_recursive();
                commands.spawn((NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb_u8(105, 105, 105)),
                    ..default()
                }, Lives)).with_children(
                    |parent| {
                        for _ in 0..player.player_data.lives {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    height: Val::Px(64.0),
                                    width: Val::Px(64.0),
                                    margin: UiRect::new(Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN), Val::Px(IMAGE_MARGIN)),
                                    ..default()
                                },
                                image: life.clone(),
                                ..default()
                            });
                        } 
                    }
                    );          
            } else {
                println!("Game Over");
                return;
            }
            // commands.entity(entity).despawn_recursive();

            }
        }
}


