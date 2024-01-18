use bevy::{prelude::*, input::{gamepad::GamepadEvent, keyboard::KeyboardInput}, window::CursorGrabMode};

pub struct InputModeManagerPlugin;

impl Plugin for InputModeManagerPlugin {
    fn build(&self, app: &mut App) {
        // Add a state to record the current active input
        app.add_state::<ActiveInput>()
            // System to switch to gamepad as active input
            .add_systems(
                Update, (
                activate_gamepad.run_if(in_state(ActiveInput::MouseKeyboard)),
                activate_mnk.run_if(in_state(ActiveInput::Gamepad)),
                grab_mouse,
            ));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ActiveInput {
    #[default]
    MouseKeyboard,
    Gamepad,
}

/// Switch the gamepad when any button is pressed or any axis input used
fn activate_gamepad(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut gamepad_event_reader: EventReader<GamepadEvent>,
) {
    for event_reader in gamepad_event_reader.read() {
        match event_reader {
            GamepadEvent::Button(_) | GamepadEvent::Axis(_) => {
                info!("Switching to gamepad input");
                next_state.set(ActiveInput::Gamepad);
                return;
            }
            _ => (),
        }
    }
}

/// Switch to mouse and keyboard input when any keyboard button is pressed
fn activate_mnk(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
) {
    for _ev in keyboard_event_reader.read() {
        info!("Switching to mouse and keyboard input");
        next_state.set(ActiveInput::MouseKeyboard);
    }
}

fn grab_mouse(mut windows: Query<&mut Window>, mouse: Res<Input<MouseButton>>, key: Res<Input<KeyCode>>,) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
