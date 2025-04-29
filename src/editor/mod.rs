mod camera_rig_orbital;
mod colors;
mod gizmos;
mod selection;
mod selection_ops;
pub mod ui;
pub mod utility;

use bevy::app::Plugin;
pub use camera_rig_orbital::CameraRigOrbital;
pub use colors::Colors;
pub use selection::Selectable;
use selection::SelectionPlugin;
use selection_ops::SelectionOpsPlugin;
use ui::EditorGuiPlugin;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(EditorGuiPlugin);
        app.add_plugins(CameraRigOrbital);
        app.add_plugins(SelectionPlugin);
        app.add_plugins(SelectionOpsPlugin);
        app.add_plugins(gizmos::GridFloorPlugin);
    }
}
