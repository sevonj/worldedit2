mod map_view;
mod outliner;
mod viewport3d;

pub use map_view::{MapViewPane, MapViewPanePlugin};
pub use outliner::{OutlinerPane, OutlinerPanePlugin};
pub use viewport3d::{ViewportPane, ViewportPanePlugin};

use bevy::prelude::*;

use bevy_egui::egui::Ui;
use egui_tiles::UiResponse;

/// Trait for building ui panes in the editor.
pub trait EditorPane {
    /// Build the ui here
    fn ui(&mut self, ui: &mut Ui, world: &mut World, commands: &mut Commands) -> UiResponse;
    fn tab_title(&self) -> &'static str;
}
