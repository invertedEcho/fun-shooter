use std::f32::consts::FRAC_PI_2;

use avian3d::{math::PI, prelude::*};
use bevy::{input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster, prelude::*};

use crate::player::{PLAYER_MOVEMENT_SPEED, components::Player};

pub fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::capsule(3.0, 3.0),
            LinearVelocity::ZERO,
            Transform::from_xyz(-10.0, 20.0, -20.0),
            Friction::new(1.0),
            LockedAxes::new()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Projection::from(PerspectiveProjection::default()),
            ));

            let model = asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("glb/weapons/WA_2000.glb#Scene0"));
            parent.spawn((
                Transform {
                    translation: Vec3 {
                        x: 2.0,
                        y: -1.0,
                        z: -5.0,
                    },
                    // rotate 180 degrees as weapon is spawned wrong way
                    // need to use radian, radian another way of representing rotation like degrees
                    // PI = 180 degrees
                    // FRAC_PI_2 (e.g. PI / 2) = 90 degrees
                    rotation: Quat::from_rotation_y(PI),
                    ..default()
                },
                SceneRoot(model),
            ));
        });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, transform) in player {
        let mut local_velocity = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            local_velocity.z -= PLAYER_MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            local_velocity.x -= PLAYER_MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            local_velocity.x += PLAYER_MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            local_velocity.z += PLAYER_MOVEMENT_SPEED;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            velocity.y = 30.0;
        }

        if local_velocity.length_squared() > 0.0 {
            let world_velocity = transform.rotation * local_velocity;
            velocity.x = world_velocity.x;
            velocity.z = world_velocity.z;
        }
    }
}

// we only need to change transform for player as camera is a child of the player
pub fn camera_orbit_player(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let delta = mouse_motion.delta;

    if delta != Vec2::ZERO {
        // pitch like nodding yes with your head
        let delta_pitch = -delta.y * 0.002;

        // yaw like nodding no with your head
        let delta_yaw = -delta.x * 0.003;

        // existing rotation
        let (current_yaw, current_pitch, current_roll) =
            player_transform.rotation.to_euler(EulerRot::YXZ);

        let new_yaw = delta_yaw + current_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let new_pitch = (delta_pitch + current_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        player_transform.rotation =
            Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, current_roll);
    }
}
