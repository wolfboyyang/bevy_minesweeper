use bevy::prelude::*;

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct BoardAssets {
    /// Label
    pub label: String,
    ///
    pub board_material: Handle<ColorMaterial>,
    ///
    pub tile_material: Handle<ColorMaterial>,
    ///
    pub covered_tile_material: Handle<ColorMaterial>,
    ///
    pub bomb_counter_font: Handle<Font>,
    ///
    pub bomb_counter_colors: Vec<Color>,
    ///
    pub flag_material: Handle<ColorMaterial>,
    ///
    pub bomb_material: Handle<ColorMaterial>,
}

impl BoardAssets {
    /// Default bomb counter color set
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }

    /// Safely retrieves the color matching a bomb counter
    pub fn bomb_counter_color(&self, counter: u8) -> Color {
        let counter = counter.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(counter) {
            Some(c) => *c,
            None => match self.bomb_counter_colors.last() {
                None => Color::WHITE,
                Some(c) => *c,
            },
        }
    }
}
