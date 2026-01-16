mod camera_rig_orbital;
mod colors;
mod gizmos;
mod resources;
mod selection;
mod selection_actions;
mod terrain_cell_preview;
mod ui;

pub use selection::Selectable;

use camera_rig_orbital::CameraRigOrbital;
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
        app.add_plugins(resources::ViewportRenderTargetPlugin);

        app.add_plugins(EditorGuiPlugin);
        app.add_plugins(CameraRigOrbital);
        app.add_plugins(SelectionPlugin);
        app.add_plugins(SelectionActionsPlugin);
        app.add_plugins(GridFloorPlugin);
        app.add_plugins(TerrainCellPreviewPlugin);
    }
}
