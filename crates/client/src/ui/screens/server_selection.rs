use bevy::{prelude::*, text::EditableText};
use netvy::NetvyMode;
use shared::{
    AppRole, StartGame, utils::network::OFFICIAL_GAME_SERVER_ADDRESS,
};

use crate::{
    game_flow::states::{
        AppState, ClientLoadingState, MainMenuState, PendingGameConfigClient,
    },
    game_settings::GameSettings,
    network::ConnectToDedicatedServer,
    ui::{
        common::{DEFAULT_FONT_SIZE, DEFAULT_GAME_FONT_PATH, DEFAULT_ROW_GAP},
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
    ConnectToOfficialServer,
    ConnectToCustomServer,
}

/// Used to display a message to the user, such as that the given port is invalid (e.g. it cant be parsed)
/// This is the marker component, to be used with `Text`
#[derive(Component)]
struct CurrentMessageText;

fn spawn_server_selection_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            ServerSelectionScreenRoot,
            Visibility::Hidden,
            Node {
                width: percent(100),
                height: percent(100),
                row_gap: DEFAULT_ROW_GAP,
                padding: UiRect::all(px(64)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Name::new("Server Selection UI Root"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select a server"),
                TextFont {
                    font: FontSource::Handle(
                        asset_server.load(DEFAULT_GAME_FONT_PATH),
                    ),
                    font_size: DEFAULT_FONT_SIZE,
                    ..default()
                },
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(32),
                    padding: UiRect::all(px(32)),
                    width: percent(100),
                    height: percent(100),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(build_left_box(
                        asset_server.load(DEFAULT_GAME_FONT_PATH),
                    ));
                    parent.spawn(build_right_box(
                        asset_server.load(DEFAULT_GAME_FONT_PATH),
                    ));
                });
            parent.spawn((Text::new(""), CurrentMessageText));
        });
}

fn build_left_box(font_handle: Handle<Font>) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(4)),
            border_radius: BorderRadius::all(px(4)),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        children![build_common_button(
            "Use official server",
            font_handle,
            ServerSelectionButton::ConnectToOfficialServer,
        )],
    )
}

fn build_right_box(font_handle: Handle<Font>) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: DEFAULT_ROW_GAP,
            border: UiRect::all(px(4)),
            padding: UiRect::all(px(8)),
            border_radius: BorderRadius::all(px(4)),
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        children![
            (Text::new("Server address:")),
            (
                Node {
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                EditableText::default(),
                CustomServerAddressInput
            ),
            (Text::new("Port:")),
            (
                Node {
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                EditableText {
                    max_characters: Some(5),
                    ..default()
                },
                CustomServerAddressInputPort,
            ),
            (build_common_button(
                "Connect",
                font_handle,
                ServerSelectionButton::ConnectToCustomServer,
            )),
        ],
    )
}

fn fill_custom_server_address_input(
    query: Query<&mut EditableText, Added<CustomServerAddressInput>>,
    game_settings: ResMut<GameSettings>,
) {
    for mut editable_text in query {
        editable_text
            .editor
            .set_text(&game_settings.server.last_custom_server_address);
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

#[derive(Component)]
struct CustomServerAddressInputPort;

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
    editable_text_server_address: Single<
        &EditableText,
        With<CustomServerAddressInput>,
    >,
    editable_text_server_port: Single<
        &EditableText,
        With<CustomServerAddressInputPort>,
    >,
    mut game_settings: ResMut<GameSettings>,
    mut current_message_text: Single<&mut Text, With<CurrentMessageText>>,
) {
    for (interaction, server_selection_button) in button_query {
        let Interaction::Pressed = interaction else {
            continue;
        };

        let port_string = editable_text_server_port.value().to_string();
        let parsed_port: u16 = match port_string.parse() {
            Ok(parsed_port) => parsed_port,
            Err(error) => {
                ***current_message_text = format!(
                    "Your given port {port_string} seems to be invalid: \
                     {error}"
                );
                error!("Given custom server port cant be parsed: {error}");
                continue;
            }
        };

        if *server_selection_button
            == ServerSelectionButton::ConnectToCustomServer
        {
            info!(
                "Storing custom server address in game settings (config file)"
            );
            game_settings.server.last_custom_server_address =
                editable_text_server_address.value().to_string();
            game_settings.server.last_custom_server_port = parsed_port;
        }

        let server_address = match server_selection_button {
            ServerSelectionButton::ConnectToOfficialServer => {
                OFFICIAL_GAME_SERVER_ADDRESS
            }
            ServerSelectionButton::ConnectToCustomServer => {
                &editable_text_server_address.value().to_string()
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
                port: parsed_port,
            },
        );

        message_writer.write(StartGame(pending_game_config.0));
    }
}
