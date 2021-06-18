use bevy::log;
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::{BoardAssets, BoardOptions, BoardPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

fn main() {
    let mut app = App::build();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 1000.,
        ..Default::default()
    })
    // Log setup
    .insert_resource(LogSettings {
        level: Level::INFO,
        ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins)
    // Board plugin
    .add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
    .add_state(AppState::Out)
    .add_startup_system(setup_board.system())
    // Startup system (cameras)
    .add_startup_system(setup_camera.system())
    // State handling
    .add_system(input_handler.system());
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // Run the app
    app.run();
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.,
        safe_start: true,
        ..Default::default()
    });
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: materials.add(Color::WHITE.into()),
        tile_material: materials.add(Color::DARK_GRAY.into()),
        covered_tile_material: materials.add(Color::GRAY.into()),
        bomb_counter_font: asset_server.load("fonts/minecraft.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: materials.add(asset_server.load("sprites/white_flag.png").into()),
        bomb_material: materials.add(asset_server.load("sprites/bomb.png").into()),
    });
    // Bevy default plugins
    state.set(AppState::InGame).unwrap();
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn input_handler(
    mut state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut board_assets: ResMut<BoardAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::S) {
        log::debug!("style switch detected");
        if &board_assets.label == "Default" {
            board_assets.label = "Dark".to_string();
            let material = materials
                .get_mut(board_assets.board_material.clone())
                .unwrap();
            material.color = Color::BLACK;
        } else {
            board_assets.label = "Default".to_string();
            let material = materials
                .get_mut(board_assets.board_material.clone())
                .unwrap();
            material.color = Color::WHITE;
        }
    }
}
