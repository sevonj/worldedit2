mod panes;
mod ui_tiling;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use ui_tiling::UiTilingPlugin;

#[derive(Debug)]
pub struct EditorGuiPlugin;

impl Plugin for EditorGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(UiTilingPlugin);

        app.add_plugins(panes::OutlinerPanePlugin);
        app.add_plugins(panes::ViewportPanePlugin);
        app.add_plugins(panes::MapViewPanePlugin);
    }
}
