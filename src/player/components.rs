use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerWeaponShootCooldownTimer(pub Timer);

#[derive(Component)]
pub struct BulletTimer(pub Timer);
