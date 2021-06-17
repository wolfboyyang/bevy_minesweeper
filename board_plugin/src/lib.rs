use crate::tile_map::TileMap;
use bevy::log;
use bevy::prelude::*;
pub use resources::*;

pub mod components;
mod resources;

pub struct BoardPlugin {}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::create_board.system());
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(mut commands: Commands, board_options: Option<Res<BoardOptions>>) {
        let options = match board_options {
            None => BoardOptions::default(), // If no options is set we use the default one
            Some(o) => {
                commands.remove_resource::<BoardOptions>(); // After this system the options are no longer relevant
                o.clone()
            }
        };

        // Tilemap generation
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.mine_count);
        #[cfg(feature = "debug")]
        // Tilemap debugging
        log::info!("{}", tile_map.console_output());
    }
}
