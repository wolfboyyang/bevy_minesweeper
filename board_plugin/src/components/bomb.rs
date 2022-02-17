use bevy::prelude::Component;

/// Bomb component
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Component)]
pub struct Bomb;
