use avian3d::prelude::*;
use bevy::{color::palettes::css::BLACK, prelude::*};

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
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

    commands.spawn((
        Collider::cuboid(10.0, 10.0, 10.0),
        Transform::from_xyz(0.0, 2.0, -30.0),
        RigidBody::Static,
    ));

    commands.spawn((
        Collider::cuboid(20.0, 10.0, 20.0),
        Transform::from_xyz(20.0, 2.0, -30.0),
        RigidBody::Static,
    ));
}
