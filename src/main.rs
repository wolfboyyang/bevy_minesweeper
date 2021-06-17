use bevy::log::{Level, LogSettings};
use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::{BoardOptions, BoardPlugin};

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
    // Board plugin options
    .insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.,
        safe_start: true,
        ..Default::default()
    })
    // Board plugin
    .add_plugin(BoardPlugin {})
    // Bevy default plugins
    .add_plugins(DefaultPlugins)
    // Startup system (cameras)
    .add_startup_system(setup_camera.system());
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // Run the app
    app.run();
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
