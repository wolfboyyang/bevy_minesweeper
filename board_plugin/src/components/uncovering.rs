#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

/// Uncovering component, indicates a tile uncovering
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct Uncovering {
    /// Remaning ticks on the uncovering animation
    pub remaining_ticks: u8,
}
