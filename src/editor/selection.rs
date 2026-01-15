use bevy::prelude::*;

use super::selection_actions::SelectionActionState;

/// Marker component for selectable entities.
#[derive(Component, Default)]
pub struct Selectable;

/// Marker component for selected entities.
#[derive(Component)]
#[require(Selectable)]
pub struct Selected;

pub type WithSelected = (With<Selectable>, With<Selected>);

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_selection);
    }
}

fn update_selection(
    mut commands: Commands,
    query: Query<(Entity, Option<&Selected>), With<Selectable>>,
    keyb: Res<ButtonInput<KeyCode>>,
    op: Res<SelectionActionState>,
) {
    if *op != SelectionActionState::None {
        return;
    }

    if keyb.just_pressed(KeyCode::Escape) {
        for (entity, ..) in query.iter() {
            commands.entity(entity).remove::<Selected>();
        }
    }
}
