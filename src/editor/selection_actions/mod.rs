mod deletion;
pub mod transform_action;

use bevy::prelude::*;

use deletion::DeletionPlugin;
use transform_action::TransformActionsPlugin;

pub struct SelectionActionsPlugin;

impl Plugin for SelectionActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectionActionState::None);
        app.add_plugins(TransformActionsPlugin);
        app.add_plugins(DeletionPlugin);
    }
}

#[derive(Debug, Resource, PartialEq, Eq)]
pub enum SelectionActionState {
    None,
    Transform,
}
