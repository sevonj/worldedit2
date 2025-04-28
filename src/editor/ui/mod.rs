mod editor_pane;
mod outliner_pane;
mod ui_tiling;
mod viewport_pane;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use outliner_pane::OutlinerPanePlugin;
use ui_tiling::UiTilingPlugin;
use viewport_pane::ViewportPanePlugin;

#[derive(Debug)]
pub struct EditorGuiPlugin;

impl Plugin for EditorGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });
        app.add_plugins(UiTilingPlugin);

        app.add_plugins(ViewportPanePlugin);
        app.add_plugins(OutlinerPanePlugin);
    }
}
