mod deletion;
pub mod transform_ops;

use deletion::DeletionPlugin;
use transform_ops::TransformOpsPlugin;

use bevy::prelude::*;

pub struct SelectionOpsPlugin;

impl Plugin for SelectionOpsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectionOpsState::None);
        app.add_plugins(TransformOpsPlugin);
        app.add_plugins(DeletionPlugin);
    }
}

#[derive(Debug, Resource, PartialEq, Eq)]
pub enum SelectionOpsState {
    None,
    Transform,
}
