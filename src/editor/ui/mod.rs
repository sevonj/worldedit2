mod main_ui;
mod viewport;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use main_ui::MainUi;
use viewport::ViewportGui;

#[derive(Debug)]
pub struct EditorGuiPlugin;

impl Plugin for EditorGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_plugins(ViewportGui);
        app.add_plugins(MainUi);
    }
}
