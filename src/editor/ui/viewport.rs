use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};

#[derive(Debug)]
pub struct ViewportGui;

impl Plugin for ViewportGui {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts) {
    Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
