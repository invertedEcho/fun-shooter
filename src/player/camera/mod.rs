use bevy::prelude::*;

use crate::player::camera::systems::{camera_orbit_player, switch_first_third_person};

pub mod components;
mod systems;

/// Everything else than player weapon is rendererd at this layer
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's weapon
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_orbit_player, switch_first_third_person));
    }
}
