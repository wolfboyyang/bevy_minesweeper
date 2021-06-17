#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

/// Bomb component
#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Bomb {}
