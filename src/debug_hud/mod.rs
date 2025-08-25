use bevy::prelude::*;

use crate::debug_hud::systems::spawn_debug_hud;

mod systems;

pub struct DebugHudPlugin;

impl Plugin for DebugHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_hud);
    }
}
