use bevy::prelude::*;

use crate::world::systems::spawn_ground;

mod systems;

pub struct WorldPlugin;

// in the future this will be the place where our procedural generation will happen. for now we
// just gonna spawn a flat ground
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}
