use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ineffable::{config::simple_asset_loading::MergeMode, plugin::IneffablePlugin, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Startup,
    Menu,
    InGame,
    EndGame,
}

#[bevy_main]
fn main() {
    run();
}

pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Your Game"),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((IneffablePlugin,));

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.register_input_action::<PlayerInput>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Startup)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("texture.assets.ron")
                .load_collection::<TextureAssets>(),
        )
        .add_systems(OnEnter(GameState::Startup), startup)
        .add_systems(OnEnter(GameState::Menu), show_ui::<MenuUi>)
        .add_systems(OnExit(GameState::Menu), hide_ui::<MenuUi>)
        .run();
}

#[derive(Resource, AssetCollection)]
struct TextureAssets {
    #[asset(key = "player")]
    player: Handle<Image>,

    #[asset(key = "player_layout")]
    player_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
struct MenuUi;

#[derive(Debug, InputAction)]
pub enum PlayerInput {
    #[ineffable(dual_axis)]
    Movement,
}

fn startup(mut commands: Commands, mut ineffable: IneffableCommands) {
    commands.spawn(Camera2dBundle::default());

    ineffable.load_configs(vec![(MergeMode::Base, "input.ron")]);
}

pub fn show_ui<T>(mut mainmenu: Query<&mut Visibility, With<T>>)
where
    T: Component,
{
    if let Ok(mut visibility) = mainmenu.get_single_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_ui<T>(mut mainmenu: Query<&mut Visibility, With<T>>)
where
    T: Component,
{
    if let Ok(mut visibility) = mainmenu.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
}
