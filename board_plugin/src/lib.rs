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
    pub fn create_board() {
        let mut tile_map = TileMap::empty(20, 20);
        tile_map.set_bombs(40);
        log::info!("{}", tile_map.console_output());
    }
}
