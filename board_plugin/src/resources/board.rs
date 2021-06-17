use crate::components::Coordinates;
use crate::tile_map::TileMap;
use crate::Bounds2;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
}

impl Board {
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        if !self.bounds.in_bounds(position) {
            return None;
        }
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    /// Retrieves a covered tile entity
    pub fn covered_tile(&self, coords: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(&coords)
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coords)
    }

    /// We retrieve the adjacent covered tile entities of `coord`
    pub fn adjacent_covered_tiles(&self, coord: &Coordinates) -> Vec<Entity> {
        let vec = self.tile_map.safe_square_at(coord);
        let mut res = Vec::new();
        for coord in vec.into_iter() {
            if let Some(entity) = self.covered_tiles.get(&coord) {
                res.push(*entity);
            }
        }
        res
    }
}
