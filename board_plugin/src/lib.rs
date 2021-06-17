use crate::components::{Bomb, BombNeighbor, Coordinates, Uncover};
use crate::events::TileTriggerEvent;
use crate::resources::tile::Tile;
use crate::tile_map::TileMap;
use bevy::log;
use bevy::prelude::*;
use bevy::text::Text2dSize;
pub use bounds::*;
pub use resources::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

mod bounds;
pub mod components;
pub mod events;
mod resources;
mod systems;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: 'static + Debug + Clone + Eq + PartialEq + Hash + Send + Sync> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        // When the running states comes into the stack we load a board
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone())
                .with_system(Self::create_board.system()),
        )
        // We handle input and trigger events only if the state is active
        .add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(systems::input::input_handling.system())
                .with_system(systems::uncover::trigger_event_handler.system()),
        )
        // We handle uncovering even if the state is inactive
        .add_system_set(
            SystemSet::on_in_stack_update(self.running_state.clone())
                .with_system(systems::uncover::uncover_tiles.system()),
        )
        .add_system_set(
            SystemSet::on_exit(self.running_state.clone())
                .with_system(Self::cleanup_board.system()),
        )
        .add_event::<TileTriggerEvent>();
    }
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<WindowDescriptor>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };

        // Tilemap generation
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        // Tilemap debugging
        log::info!("{}", tile_map.console_output());

        // Setup

        // We define the size of our tiles in world space
        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };
        // We deduce the size of the complete board
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        // TODO: refactor this (This will move into a resource in a following chapter)
        let board_material = materials.add(Color::WHITE.into());
        let tile_material = materials.add(Color::DARK_GRAY.into());
        let covered_tile_material = materials.add(Color::GRAY.into());
        let font = asset_server.load("fonts/minecraft.ttf");
        let bomb_material = materials.add(asset_server.load("sprites/bomb.png").into());
        //

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        let mut safe_start = None;
        let board_entity = commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            // This component is required until https://github.com/bevyengine/bevy/pull/2331 is merged
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite::new(board_size),
                        material: board_material,
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    tile_material,
                    bomb_material,
                    covered_tile_material,
                    font,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();
        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover {});
            }
        }
        // We add the main resource of the game, the board
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.into(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
        })
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        material: Handle<ColorMaterial>,
        bomb_material: Handle<ColorMaterial>,
        covered_tile_material: Handle<ColorMaterial>,
        font: Handle<Font>,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        // Tiles
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut cmd = parent.spawn();
                let mut covered_tile_entity = None;
                // Tile sprite
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::splat(size - padding)),
                    material: material.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                // Tile name
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                // Tile coordinates
                .insert(coordinates)
                // Children
                .with_children(|parent| {
                    let mut child_cmd = parent.spawn();
                    // Tile cover
                    covered_tile_entity = Some(
                        child_cmd
                            .insert_bundle(SpriteBundle {
                                sprite: Sprite::new(Vec2::splat(size - padding)),
                                transform: Transform::from_xyz(0., 0., 2.),
                                material: covered_tile_material.clone(),
                                ..Default::default()
                            })
                            .id(),
                    );
                    covered_tiles.insert(coordinates, covered_tile_entity.unwrap());
                });
                match tile {
                    // If the tile is a bomb we add the matching component and a sprite child
                    Tile::Bomb => {
                        cmd.insert(Bomb {});
                        cmd.with_children(|child_cmd| {
                            child_cmd.spawn_bundle(SpriteBundle {
                                sprite: Sprite::new(Vec2::splat(size - padding)),
                                transform: Transform::from_xyz(0., 0., 1.),
                                material: bomb_material.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    // If the tile is a bomb neighbour we add the matching component and a text child
                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor { count: *v });
                        cmd.with_children(|child_cmd| {
                            child_cmd.spawn_bundle(Self::bomb_count_text_bundle(
                                *v,
                                font.clone(),
                                size - padding,
                            ));
                        });
                    }
                    Tile::Empty => {
                        if safe_start_entity.is_none() {
                            *safe_start_entity = covered_tile_entity
                        }
                    }
                }
            }
        }
    }

    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        // We retrieve the text and the correct color
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );
        // We generate a text bundle
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            text_2d_size: Text2dSize {
                size: Size {
                    width: size,
                    height: size,
                },
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn adaptative_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_heigth = window.height / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
