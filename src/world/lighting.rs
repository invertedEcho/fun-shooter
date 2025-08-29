use bevy::{prelude::*, render::view::RenderLayers};

use crate::player::camera::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        // PointLight {
        //     color: Color::WHITE,
        //     shadows_enabled: true,
        //     ..default()
        // },
        Transform::from_xyz(0.0, 8.0, 0.0),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}
