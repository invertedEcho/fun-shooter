use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{game_flow::GameFlowPlugin, player::PlayerPlugin, world::WorldPlugin};

mod game_flow;
mod player;
mod world;

fn main() {
    let mut app = App::new();

    // bevy-builtin plugins
    app.add_plugins(DefaultPlugins);

    // avian (physics)
    app.add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec3::NEG_Y * 100.0));

    // own plugins
    app.add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(GameFlowPlugin);

    app.run();
}
