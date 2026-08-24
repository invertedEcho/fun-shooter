use bevy::prelude::*;
use netvy::prelude::*;
use shared::{
    AppRole, StopGame, enemy::components::Enemy, game_score::GameScore,
    player::Player,
};

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
            ),
        );
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

fn remove_player_game_score(
    mut client_disconnected: MessageReader<FromServer<ClientDisconnected>>,
    mut game_score: Query<&mut GameScore>,
    app_role: Res<State<AppRole>>,
) {
    // GameScore is only spawned on server
    if *app_role.get() == AppRole::ClientOnly {
        return;
    }

    for message in client_disconnected.read() {
        match game_score.single_mut() {
            Ok(mut game_score) => {
                game_score.players.remove(&message.0.client);
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
        for entity in game_score {
            commands.entity(entity).despawn();
        }
    }
}
