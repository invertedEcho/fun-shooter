use bevy::prelude::*;

use crate::game_flow::systems::grab_mouse;

mod systems;

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grab_mouse);
    }
}
