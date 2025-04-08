use bevy::prelude::*;
use bevy_egui::egui::Ui;
use egui_tiles::UiResponse;

/// Trait for building ui panes in the editor.
pub trait EditorPane {
    /// Build the ui here
    fn ui(&mut self, ui: &mut Ui, world: &mut World) -> UiResponse;
    /// Tab title text
    fn title(&self) -> String;
}
