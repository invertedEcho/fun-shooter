use bevy::{
    color::palettes::tailwind::SLATE_300,
    prelude::*,
    text::{EditableText, TextCursorStyle},
};
use netvy::NetvyMode;
use shared::{AppRole, StartGame, utils::network::OFFICIAL_GAME_SERVER};

use crate::{
    game_flow::states::{
        AppState, ClientLoadingState, MainMenuState, PendingGameConfigClient,
    },
    game_settings::GameSettings,
    network::ConnectToDedicatedServer,
    ui::{
        common::{DEFAULT_GAME_FONT_PATH, DEFAULT_ROW_GAP},
        widgets::button::build_common_button,
    },
};

pub struct ServerSelectionScreenPlugin;

impl Plugin for ServerSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_server_selection_screen);

        app.add_systems(
            Update,
            (handle_buttons_interaction, fill_custom_server_address_input),
        );

        app.add_systems(OnEnter(MainMenuState::ServerSelection), show_screen);
        app.add_systems(OnExit(MainMenuState::ServerSelection), hide_screen);
    }
}

#[derive(Component)]
struct ServerSelectionScreenRoot;

#[derive(Component, PartialEq)]
enum ServerSelectionButton {
    ConnectToDedicatedServer,
    ConnectToCustomServer,
}

fn spawn_server_selection_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            ServerSelectionScreenRoot,
            Visibility::Hidden,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: percent(100),
                height: percent(100),
                row_gap: DEFAULT_ROW_GAP,
                ..default()
            },
            Name::new("Server Selection UI Root"),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Select a server"));
            parent.spawn(build_common_button(
                "Play on official server",
                asset_server.load(DEFAULT_GAME_FONT_PATH),
                ServerSelectionButton::ConnectToDedicatedServer,
            ));
            parent.spawn(build_common_button(
                "Play on custom server",
                asset_server.load(DEFAULT_GAME_FONT_PATH),
                ServerSelectionButton::ConnectToCustomServer,
            ));
            parent.spawn(Text::new(
                "Type in the server address to which you want to connect to",
            ));
            parent.spawn((
                EditableText {
                    allow_newlines: false,
                    ..default()
                },
                Name::new("CustomServerAddressInput"),
                TextCursorStyle::default(),
                CustomServerAddressInput,
                Node {
                    border: px(2).all(),
                    width: percent(30),
                    ..Default::default()
                },
                BorderColor::from(Color::from(SLATE_300)),
            ));
        });
}

fn fill_custom_server_address_input(
    query: Query<&mut EditableText, Added<CustomServerAddressInput>>,
    game_settings: ResMut<GameSettings>,
) {
    for mut editable_text in query {
        editable_text
            .editor
            .set_text(&game_settings.server.last_custom_server);
    }
}

fn show_screen(
    visibility: Single<&mut Visibility, With<ServerSelectionScreenRoot>>,
) {
    info!("Making ServerSelectionScreenRoot visible");
    *visibility.into_inner() = Visibility::Visible;
}

fn hide_screen(
    visibility: Single<&mut Visibility, With<ServerSelectionScreenRoot>>,
) {
    info!("Making ServerSelectionScreenRoot hidden");
    *visibility.into_inner() = Visibility::Hidden;
}

#[derive(Component)]
struct CustomServerAddressInput;

fn handle_buttons_interaction(
    button_query: Query<
        (&Interaction, &ServerSelectionButton),
        Changed<Interaction>,
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_app_role: ResMut<NextState<AppRole>>,
    mut next_client_loading_state: ResMut<NextState<ClientLoadingState>>,
    mut message_writer: MessageWriter<StartGame>,
    mut connect_to_dedicated_server_message_writer: MessageWriter<
        ConnectToDedicatedServer,
    >,
    mut netvy_mode: ResMut<NetvyMode>,
    pending_game_config: Res<PendingGameConfigClient>,
    editable_text_query: Single<&EditableText, With<CustomServerAddressInput>>,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, server_selection_button) in button_query {
        let Interaction::Pressed = interaction else {
            continue;
        };

        if *server_selection_button
            == ServerSelectionButton::ConnectToCustomServer
        {
            info!(
                "Storing custom server address in game settings (config file)"
            );
            game_settings.server.last_custom_server =
                editable_text_query.value().to_string();
        }

        let server_address = match server_selection_button {
            ServerSelectionButton::ConnectToDedicatedServer => {
                OFFICIAL_GAME_SERVER
            }
            ServerSelectionButton::ConnectToCustomServer => {
                &editable_text_query.value().to_string()
            }
        };

        // TODO: All this shouldnt happen in the UI, write a message instead that gets
        // handled somewhere where it makes more sense, probably game_core.
        *netvy_mode = NetvyMode::Client;
        next_app_state.set(AppState::LoadingGame);
        next_app_role.set(AppRole::ClientOnly);
        // NOTE: we skip state StartingServer, because in multiplayer we dont start a
        // server ourself but connect to the dedicated server
        next_client_loading_state.set(ClientLoadingState::ConnectingToServer);

        connect_to_dedicated_server_message_writer.write(
            ConnectToDedicatedServer {
                server_address: server_address.to_string(),
            },
        );

        message_writer.write(StartGame(pending_game_config.0));
    }
}
