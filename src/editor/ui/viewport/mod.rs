mod rt;

use bevy::prelude::*;
use bevy_egui::{
    egui::{CentralPanel, Frame, Window},
    EguiContextPass, EguiContexts,
};

use crate::editor::{selection::Selected, selection_ops::transform_ops::TransformOp};

pub use rt::ViewportRT;

#[derive(Debug)]
pub struct ViewportGui;

impl Plugin for ViewportGui {
    fn build(&self, app: &mut App) {
        rt::build(app);
        app.add_systems(
            EguiContextPass,
            (
                draw_viewport_ui,
                camera_controls_ui,
                selection_ui,
                xform_ops_ui,
            )
                .chain(),
        );
    }
}

fn draw_viewport_ui(
    mut contexts: EguiContexts,
    viewport_img: Res<ViewportRT>,
    images: ResMut<Assets<Image>>,
) {
    let viewport_tex_id = contexts.image_id(&viewport_img).unwrap();
    let ctx = contexts.ctx_mut();

    CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
        rt::viewport_rt_ui(viewport_img, images, viewport_tex_id, ui);
    });
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
                1 => "1 entity selected".to_string(),
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
                | TransformOp::Rotate { axis_lock, .. }
                | TransformOp::Scale(axis_lock) => {
                    ui.label("Esc: cancel selection");
                    ui.label(format!("axis: {axis_lock}"));
                }
            }
        });
}
