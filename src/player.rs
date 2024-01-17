use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{axislike::DualAxisData, action_state::ActionState, plugin::InputManagerPlugin};

use crate::{PlayerAction, input::{ActiveInput, InputModeManagerPlugin}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputModeManagerPlugin);
        // Defined below, detects whether MKB or gamepad are active
        app.init_resource::<ActionState<PlayerAction>>();
        app.insert_resource(PlayerAction::default_input_map());

        app
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                player_mouse_look.run_if(in_state(ActiveInput::MouseKeyboard)),
            )
            .add_systems(Update, control_player.after(player_mouse_look));
    } 
}

// A player marker
#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();

    let texture: Handle<Image> = asset_server.load("sprites/Player.png");

    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        texture: texture, 
        ..default()
    }, Player {},));

    // commands.spawn(InputManagerBundle::<PlayerAction> {
        // action_state: ActionState::default(),
        // input_map: InputMap::new([KeyCode::W, Action::Move])
    // });
}

fn player_mouse_look(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut action_state: ResMut<ActionState<PlayerAction>>,
) {
    // Update each actionstate with the mouse position from the window
    // by using the referenced entities in ActionStateDriver and the stored action as
    // a key into the action data
    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let player_transform = player_query.get_single().expect("Need a single player");
    let window = window_query
        .get_single()
        .expect("Need a single primary window");

    // Many steps can fail here, so we'll wrap in an option pipeline
    // First check if cursor is in window
    // Then check if the ray intersects the plane defined by the player
    // Then finally compute the point along the ray to look at
    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .and_then(|ray| Some(ray).zip(ray.intersect_plane(player_transform.translation, Vec3::Y)))
        .map(|(ray, p)| ray.get_point(p))
    {
        let diff = (p - player_transform.translation).xz();
        if diff.length_squared() > 1e-3f32 {
            // Press the look action, so we can check that it is active
            action_state.press(PlayerAction::Look);
            // Modify the action data to set the axis
            let action_data = action_state.action_data_mut(PlayerAction::Look);
            // Flipping y sign here to be consistent with gamepad input. We could also invert the gamepad y axis
            action_data.axis_pair = Some(DualAxisData::from_xy(Vec2::new(diff.x, -diff.y)));
        }
    }
}

fn control_player(
    time: Res<Time>,
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    if action_state.pressed(PlayerAction::Move) {
        // Note: In a real game we'd feed this into an actual player controller
        // and respects the camera extrinsics to ensure the direction is correct
        let move_delta = time.delta_seconds()
            * action_state
                .clamped_axis_pair(PlayerAction::Move)
                .unwrap()
                .xy();
        player_transform.translation += Vec3::new(move_delta.x, 0.0, move_delta.y);
        println!("Player moved to: {}", player_transform.translation.xz());
    }

    if action_state.pressed(PlayerAction::Look) {
        let look = action_state
            .axis_pair(PlayerAction::Look)
            .unwrap()
            .xy()
            .normalize();
        println!("Player looking in direction: {}", look);
    }

    if action_state.pressed(PlayerAction::Shoot) {
        println!("Shoot!")
    }
}
