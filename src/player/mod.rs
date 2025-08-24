use bevy::prelude::*;

use crate::player::systems::{camera_orbit_player, player_movement, spawn_player};

pub mod components;
mod systems;

const PLAYER_MOVEMENT_SPEED: f32 = 20.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, camera_orbit_player));
    }
}
