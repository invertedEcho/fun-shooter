use avian3d::prelude::*;
use bevy::{color::palettes::css::BLACK, prelude::*};

pub fn spawn_world(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 4000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));

    commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        Friction::new(1.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3 {
                x: 50.0,
                y: 0.5,
                z: 50.0,
            },
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })),
        RigidBody::Static,
    ));

    let model =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("maps/fps_tps_map.glb#Scene0"));
    commands
        .spawn(Transform::from_xyz(0.0, 0.0, 0.0))
        .with_children(|parent| {
            parent.spawn(SceneRoot(model));
        });
}
