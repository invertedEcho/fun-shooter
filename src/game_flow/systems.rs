use bevy::{prelude::*, window::CursorGrabMode};

pub fn grab_mouse(mut window: Single<&mut Window>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        window.cursor_options.visible = !window.cursor_options.visible;
        window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        }
    }
}
