mod buttons;

use bevy::log;
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::buttons::{ButtonAction, ButtonMaterials};
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectableRegistry;
use board_plugin::{BoardAssets, BoardOptions, BoardPlugin, BoardPosition};

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
        height: 750.,
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
    // UI
    .add_startup_system(setup_ui.system())
    // State handling
    .add_system(input_handler.system());
    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    #[cfg(feature = "debug")]
    {
        // getting registry from world
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);
        // registering custom component to be able to edit it in inspector
        registry.register::<ButtonAction>();
    }
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
        bomb_count: 50,
        tile_padding: 1.,
        safe_start: true,
        position: BoardPosition::Centered {
            offset: Vec3::new(0., 25., 0.),
        },
        ..Default::default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: materials.add(Color::WHITE.into()),
        tile_material: materials.add(Color::DARK_GRAY.into()),
        covered_tile_material: materials.add(Color::GRAY.into()),
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: materials.add(asset_server.load("sprites/flag.png").into()),
        bomb_material: materials.add(asset_server.load("sprites/bomb.png").into()),
    });
    // Launch game
    state.set(AppState::InGame).unwrap();
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // UI Camera
    commands.spawn_bundle(UiCameraBundle::default());
}

#[allow(clippy::type_complexity)]
fn input_handler(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<AppState>>,
    mut board_assets: ResMut<BoardAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (interaction, mut material, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match action {
                    ButtonAction::Clear => {
                        log::debug!("clearing detected");
                        if state.current() == &AppState::InGame {
                            log::info!("clearing game");
                            state.set(AppState::Out).unwrap();
                        }
                    }
                    ButtonAction::Generate => {
                        log::debug!("loading detected");
                        if state.current() == &AppState::Out {
                            log::info!("loading game");
                            state.set(AppState::InGame).unwrap();
                        }
                    }
                    ButtonAction::SwitchTheme => {
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
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let button_materials = ButtonMaterials {
        normal: materials.add(Color::GRAY.into()),
        hovered: materials.add(Color::DARK_GRAY.into()),
        pressed: materials.add(Color::BLACK.into()),
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(50.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        })
        .insert(Name::new("UI"))
        .with_children(|parent| {
            let font = asset_server.load("fonts/pixeled.ttf");
            setup_single_menu(
                parent,
                "CLEAR",
                button_materials.normal.clone(),
                font.clone(),
                ButtonAction::Clear,
            );
            setup_single_menu(
                parent,
                "GENERATE",
                button_materials.normal.clone(),
                font.clone(),
                ButtonAction::Generate,
            );
            setup_single_menu(
                parent,
                "SWITCH THEME",
                button_materials.normal.clone(),
                font,
                ButtonAction::SwitchTheme,
            );
        });
    commands.insert_resource(button_materials);
}

fn setup_single_menu(
    parent: &mut ChildBuilder,
    text: &str,
    material: Handle<ColorMaterial>,
    font: Handle<Font>,
    action: ButtonAction,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(95.), Val::Auto),
                margin: Rect::all(Val::Px(10.)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .insert(action)
        .insert(Name::new(text.to_string()))
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text.to_string(),
                        style: TextStyle {
                            font,
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                ..Default::default()
            });
        });
}
