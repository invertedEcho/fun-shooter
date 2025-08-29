use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    debug_hud::DebugHudPlugin, game_flow::GameFlowPlugin, player::PlayerPlugin, world::WorldPlugin,
};

mod debug_hud;
mod game_flow;
mod player;
mod world;

fn main() {
    let mut app = App::new();

    // bevy-builtin plugins
    app.add_plugins(DefaultPlugins);

    // avian (physics)
    app.add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default());

    // own plugins
    app.add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(GameFlowPlugin)
        .add_plugins(DebugHudPlugin);

    // misc plugins
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.run();
}
