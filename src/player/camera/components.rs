use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera {
    pub camera_mode: PlayerCameraMode,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera {
            camera_mode: PlayerCameraMode::FirstPerson,
        }
    }
}

pub enum PlayerCameraMode {
    FirstPerson,
    ThirdPerson,
}
