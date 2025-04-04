use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts};

use crate::editor::{selection::Selected, transform_ops::TransformOp};

#[derive(Debug)]
pub struct ViewportGui;

impl Plugin for ViewportGui {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (camera_controls_ui, selection_ui, xform_ops_ui).chain(),
        );
    }
}

fn camera_controls_ui(mut contexts: EguiContexts) {
    Window::new("Camera")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("MMB: Move camera");
            ui.label("Shift + MMB: Rotate camera");
        });
}

fn selection_ui(mut contexts: EguiContexts, selection: Query<Entity, With<Selected>>) {
    Window::new("Selection")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            let num_selected = selection.iter().count();
            let selection_text = match num_selected {
                0 => "Nothing selected".to_string(),
                1 => format!("1 entity selected"),
                n => format!("{n} entities selected"),
            };

            ui.label(selection_text);

            if num_selected > 0 {
                ui.label("Esc: cancel selection");
            }
        });
}

fn xform_ops_ui(mut contexts: EguiContexts, op: Res<TransformOp>) {
    Window::new("Transform Ops")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!("{}", *op));

            match op.as_ref() {
                TransformOp::None => {
                    ui.label("G: Move");
                    ui.label("R: Rotate");
                    ui.label("S: Scale");
                }
                TransformOp::Move { axis_lock, .. }
                | TransformOp::Rotate(axis_lock)
                | TransformOp::Scale(axis_lock) => {
                    ui.label("Esc: cancel selection");
                    ui.label(format!("axis: {axis_lock}"));
                }
            }
        });
}
