#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

/// Uncover component, indicates a covered tile that should be uncovered
#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Uncover {}
