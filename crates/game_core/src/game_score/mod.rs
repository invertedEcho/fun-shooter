use bevy::{platform::collections::HashMap, prelude::*};
use netvy::prelude::*;
use shared::{
    AppRole, StartGame, StopGame, enemy::components::Enemy,
    game_score::GameScore, player::Player,
};

use crate::GameCoreLoadingState;

pub struct GameScorePlugin;

impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AddKillAndDeathGameScore>();

        app.add_systems(
            FixedUpdate,
            (
                update_game_score,
                remove_player_game_score,
                despawn_game_score,
                spawn_game_score,
            ),
        );
    }
}

fn spawn_game_score(
    mut commands: Commands,
    app_role: Res<State<AppRole>>,
    mut next_game_core_loading_state: ResMut<NextState<GameCoreLoadingState>>,
    mut message_reader: MessageReader<StartGame>,
) {
    for _ in message_reader.read() {
        if *app_role.get() != AppRole::ClientOnly {
            info!("Received StartGame message, spawning GameScore");
            // so server spawns this. it gets replicated to client. but somehow it wont get synced
            // again to a client reconnectin, e.g. stopping game and then joining again?
            commands.spawn((
                GameScore {
                    players: HashMap::new(),
                    enemies: HashMap::new(),
                },
                Name::new("Game Score"),
                ReplicateEntity,
            ));
        } else {
            info!(
                "Not spawning GameScore on StartGame message, we are ClientOnly."
            );
        }

        // theoretically the game score entity is not necessarily already spawned here, but we
        // just do it here as spawning such a simple entity is trivial.
        next_game_core_loading_state
            .set(GameCoreLoadingState::GameScoreFinishedSetup);
    }
}

#[derive(Message)]
pub struct AddKillAndDeathGameScore {
    pub entity_that_shot: Entity,
    pub entity_killed: Entity,
}

fn update_game_score(
    mut message_reader: MessageReader<AddKillAndDeathGameScore>,
    player_query: Query<&Owner, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut game_score: Single<&mut GameScore>,
) {
    for AddKillAndDeathGameScore {
        entity_that_shot,
        entity_killed,
    } in message_reader.read()
    {
        if let Ok(player_owner) = player_query.get(*entity_killed) {
            let Some(score) = game_score.players.get_mut(&player_owner.0)
            else {
                error!(
                    "Failed to update GameScore, {:?} doesnt exist in \
                     game_score.players",
                    player_owner
                );
                continue;
            };
            score.deaths += 1;
        } else if let Ok(enemy) = enemy_query.get(*entity_killed) {
            let Some(score) = game_score.enemies.get_mut(&enemy) else {
                error!(
                    "Failed to update GameScore, {:?} doesnt exist in \
                     game_score.enemies",
                    enemy
                );
                continue;
            };
            score.deaths += 1;
        } else {
            error!(
                "Failed to updat GameScore, entity_killed {entity_killed} is \
                 neither an enemy nor a player"
            );
        }

        if let Ok(player_owner) = player_query.get(*entity_that_shot) {
            let Some(score) = game_score.players.get_mut(&player_owner.0)
            else {
                error!(
                    "Failed to update GameScore, {:?} doesnt exist in \
                     game_score.players",
                    player_owner
                );
                continue;
            };
            score.kills += 1;
        } else if let Ok(enemy) = enemy_query.get(*entity_that_shot) {
            let Some(score) = game_score.enemies.get_mut(&enemy) else {
                error!(
                    "Failed to update GameScore, {:?} doesnt exist in \
                     game_score.enemies",
                    enemy
                );
                continue;
            };
            score.kills += 1;
        } else {
            error!(
                "Failed to updat GameScore, entity_killed {entity_killed} is \
                 neither an enemy nor a player"
            );
        }
    }
}

// No need to do this on the client too, as GameScore will get synced to clients
fn remove_player_game_score(
    mut client_disconnected: MessageReader<ClientDisconnectedServer>,
    mut game_score: Query<&mut GameScore>,
) {
    for message in client_disconnected.read() {
        info!(
            "Client {:?} disconnected, removing its player from GameScore",
            message.client
        );
        match game_score.single_mut() {
            Ok(mut game_score) => {
                game_score.players.remove(&message.client);
            }
            Err(error) => {
                error!(
                    ?error,
                    "Received ClientDisconnected but couldnt get GameScore, \
                     player wont be removed from GameScore"
                );
            }
        }
    }
}

fn despawn_game_score(
    mut commands: Commands,
    mut message_reader: MessageReader<StopGame>,
    game_score: Query<Entity, With<GameScore>>,
) {
    for _ in message_reader.read() {
        info!("Read StopGame, despawning all GameScore");
        for entity in game_score {
            info!("Despawning GameScore entity {entity}");
            commands.entity(entity).despawn();
        }
    }
}
