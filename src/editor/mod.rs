mod camera_rig_orbital;
mod gizmos;
mod ops;
mod selection;
mod transform_ops;
pub mod ui;

use bevy::app::Plugin;
pub use camera_rig_orbital::CameraRigOrbital;
pub use selection::Selectable;
use selection::SelectionPlugin;
use transform_ops::TransformOpsPlugin;
use ui::EditorGuiPlugin;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(EditorGuiPlugin);
        app.add_plugins(CameraRigOrbital);
        app.add_plugins(SelectionPlugin);
        app.add_plugins(TransformOpsPlugin);
        app.add_plugins(gizmos::GridFloorPlugin);
    }
}
