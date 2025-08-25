use bevy::prelude::*;

use crate::player::{
    camera::PlayerCameraPlugin,
    systems::{
        basic_shooting, handle_bullet_timer, player_movement, spawn_player,
        tick_player_weapon_timer,
    },
};

mod camera;
pub mod components;
mod systems;

const PLAYER_WALK_SPEED: f32 = 2.0;
const PLAYER_RUN_SPEED: f32 = 5.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCameraPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    basic_shooting,
                    tick_player_weapon_timer,
                    handle_bullet_timer,
                ),
            );
    }
}
