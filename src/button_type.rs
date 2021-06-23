#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

/// Button action type
#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ButtonAction {
    Clear,
    Generate,
    SwitchTheme,
}
