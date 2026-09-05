use bevy::prelude::*;
use netvy::prelude::*;
use shared::{
    player::{OurPlayerReady, Player},
    shooting::{PlayerWeapons, WeaponKind},
};

use crate::player::{
    camera::{PlayerCameraPlugin, components::PlayerWeaponModel},
    shooting::{
        PlayerShootingPlugin, asset_paths::get_path_to_model_for_weapon_kind,
    },
};

pub mod camera;
pub mod shooting;

#[derive(Message)]
pub struct PlayerDeathMessage;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (mark_players_as_ready, add_player_weapon_model_on_new_player),
        )
        .add_plugins(PlayerCameraPlugin)
        .add_plugins(PlayerShootingPlugin);
    }
}

type PlayersWithoutReadyMarker = (
    With<Player>,
    With<PlayerWeapons>,
    // we only insert PlayerReady component into our own player.
    With<Owned>,
    Without<OurPlayerReady>,
);

// hmm should this run on client?
fn mark_players_as_ready(
    mut commands: Commands,
    query: Query<Entity, PlayersWithoutReadyMarker>,
) {
    for entity in query {
        debug!("Marking player {entity} as ready");
        commands.entity(entity).insert(OurPlayerReady);
    }
}

fn add_player_weapon_model_on_new_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Owner, &NetEntityId), Added<Player>>,
    our_peer_id: If<Res<OurPeerId>>,
) {
    let weapon_model_path =
        get_path_to_model_for_weapon_kind(&WeaponKind::AK47);
    let weapon_model = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset(weapon_model_path));

    for (player_entity, owner, net_entity_id) in player_query {
        // we dont add player weapon model to our own player as we already do that elsewhere, with
        // different handling
        if owner.0.0 == our_peer_id.0.0.0 {
            continue;
        }
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                Name::new("PlayerWeaponModel"),
                WorldAssetRoot(weapon_model.clone()),
                Transform {
                    translation: vec3(0.2, 0.1, -0.1),
                    ..default()
                },
                PlayerWeaponModel,
                Visibility::Visible,
                AlternateTargetRotation(*net_entity_id),
            ));
        });
    }
}
