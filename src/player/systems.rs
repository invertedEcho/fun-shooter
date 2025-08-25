use std::time::Duration;

use avian3d::{
    math::PI,
    prelude::{mass_properties::components, *},
};
use bevy::{color::palettes::css::RED, prelude::*};

use crate::player::{
    PLAYER_MOVEMENT_SPEED,
    camera::components::PlayerCamera,
    components::{BulletTimer, Player, PlayerWeaponShootCooldownTimer},
};

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
                Transform::from_xyz(0.0, 0.0, 0.0),
                Projection::from(PerspectiveProjection::default()),
                PlayerCamera::default(),
            ));

            let model = asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("weapons/rifle/WA_2000.glb#Scene0"));
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

pub fn basic_shooting(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_transform: Single<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_weapon_shoot_cooldown_timer_query: Query<&PlayerWeaponShootCooldownTimer>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    // if on cooldown, dont allow shooting
    if player_weapon_shoot_cooldown_timer_query.iter().len() != 0 {
        return;
    }

    // if no timer, means we are allowed to shoot, and insert the cooldown timer
    commands.spawn(PlayerWeaponShootCooldownTimer(Timer::from_seconds(
        0.1,
        TimerMode::Once,
    )));

    let audio = asset_server
        .load("weapons/Snake's Authentic Gun Sounds/Full Sound/7.62x39/MP3/762x39 Single MP3.mp3");

    commands.spawn((AudioPlayer::new(audio), PlaybackSettings::ONCE));

    let local_bullet_velocity = Vec3 {
        z: -100.0,
        x: 0.0,
        y: 0.0,
    };
    let world_bullet_velocity = player_transform.rotation * local_bullet_velocity;

    commands.spawn((
        Transform {
            translation: Vec3 {
                x: player_transform.translation.x,
                y: player_transform.translation.y,
                z: player_transform.translation.z,
            },
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        Sensor,
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3::splat(0.25),
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        LinearVelocity(world_bullet_velocity),
        RigidBody::Kinematic,
        BulletTimer(Timer::from_seconds(3.0, TimerMode::Once)),
    ));
}

pub fn tick_player_weapon_timer(
    mut commands: Commands,
    query: Query<(Entity, &mut PlayerWeaponShootCooldownTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_bullet_timer(
    bullet_timer_query: Query<(Entity, &mut BulletTimer)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut timer) in bullet_timer_query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
