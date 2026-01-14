mod camera_rig_orbital;
mod colors;
mod gizmos;
mod selection;
mod selection_ops;
mod terrain_cell_preview;
mod ui;
mod utility;

pub use selection::Selectable;

use camera_rig_orbital::CameraRigOrbital;
use colors::Colors;
use gizmos::GridFloorPlugin;
use selection::SelectionPlugin;
use selection_ops::SelectionOpsPlugin;
use terrain_cell_preview::TerrainCellPreviewPlugin;
use ui::EditorGuiPlugin;

use bevy::app::Plugin;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(EditorGuiPlugin);
        app.add_plugins(CameraRigOrbital);
        app.add_plugins(SelectionPlugin);
        app.add_plugins(SelectionOpsPlugin);
        app.add_plugins(GridFloorPlugin);
        app.add_plugins(TerrainCellPreviewPlugin);
    }
}
