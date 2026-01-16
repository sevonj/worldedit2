mod camera_rig_orbital;
mod camera_rig_topdown;
mod colors;
mod components;
mod gizmos;
mod selection;
mod selection_actions;
mod terrain_cell_preview;
mod ui;

pub use selection::Selectable;

use camera_rig_orbital::CameraRigOrbital;
use camera_rig_topdown::CameraRigTopdown;
use colors::Colors;
use gizmos::GridFloorPlugin;
use selection::SelectionPlugin;
use selection_actions::SelectionActionsPlugin;
use terrain_cell_preview::TerrainCellPreviewPlugin;
use ui::EditorGuiPlugin;

use bevy::app::Plugin;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(components::ViewportRenderTargetPlugin);

        app.add_plugins(EditorGuiPlugin);
        app.add_plugins(CameraRigOrbital);
        app.add_plugins(CameraRigTopdown);
        app.add_plugins(SelectionPlugin);
        app.add_plugins(SelectionActionsPlugin);
        app.add_plugins(GridFloorPlugin);
        app.add_plugins(TerrainCellPreviewPlugin);
    }
}
