use bevy::prelude::*;

use crate::player::camera::systems::{camera_orbit_player, switch_first_third_person};

pub mod components;
mod systems;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_orbit_player, switch_first_third_person));
    }
}
