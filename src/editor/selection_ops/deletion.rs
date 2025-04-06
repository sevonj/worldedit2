use bevy::prelude::*;

use crate::editor::selection::WithSelected;

use super::SelectionOpsState;

pub struct DeletionPlugin;

impl Plugin for DeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    mut commands: Commands,
    selection: Query<Entity, WithSelected>,
    selection_state: ResMut<SelectionOpsState>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    if *selection_state != SelectionOpsState::None {
        return;
    }

    if kb.just_pressed(KeyCode::Delete) {
        for entity in selection.iter() {
            commands.entity(entity).despawn();
        }
    }
}
