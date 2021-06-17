#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

/// Bomb neighbor component
#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BombNeighbor {
    /// Number of neighbor bombs
    pub count: u8,
}
