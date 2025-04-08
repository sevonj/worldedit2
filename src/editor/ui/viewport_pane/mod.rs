mod render_target;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Frame},
    EguiContexts,
};

use super::{
    editor_pane::EditorPane,
    ui_tiling::{TileTree, TilingPane},
};
use crate::editor::{selection::WithSelected, selection_ops::transform_ops::TransformOp};

pub use render_target::ViewportRT;

#[derive(Debug)]
pub struct ViewportPanePlugin;

impl Plugin for ViewportPanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, render_target::create_viewport_img);
        app.add_systems(Startup, register_pane);
        app.add_systems(PreUpdate, render_target::refresh_camera_target);
        app.add_systems(PostUpdate, render_target::update_viewport_img_size);
    }
}

#[derive(Debug)]
pub struct ViewportPane {
    viewport_tex_id: egui::TextureId,
    /// Available UI size from last update
    size: egui::Vec2,
}

impl ViewportPane {
    pub fn new(viewport_tex_id: egui::TextureId) -> Self {
        Self {
            viewport_tex_id,
            size: egui::vec2(0., 0.),
        }
    }

    pub const fn size(&self) -> egui::Vec2 {
        self.size
    }
}

impl EditorPane for ViewportPane {
    fn ui(&mut self, ui: &mut bevy_egui::egui::Ui, world: &mut World) -> egui_tiles::UiResponse {
        self.size = ui.available_size();

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show_inside(ui, |ui| {
                egui::TopBottomPanel::bottom("viewport_bottom").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        selection_ui(ui, world);
                    });
                });

                let image = egui::Image::new(egui::load::SizedTexture::new(
                    self.viewport_tex_id,
                    self.size,
                ));
                image.paint_at(ui, ui.available_rect_before_wrap());

                camera_controls_ui(ui, world);
            });

        egui_tiles::UiResponse::None
    }

    fn title(&self) -> String {
        "Scene".into()
    }
}

fn register_pane(
    mut tree: ResMut<TileTree>,
    contexts: EguiContexts,
    viewport_img: Res<ViewportRT>,
) {
    let viewport_tex_id = contexts.image_id(&viewport_img).unwrap();
    let pane = TilingPane::ViewPort(ViewportPane::new(viewport_tex_id));
    tree.register_pane(pane);
}

fn camera_controls_ui(ui: &mut egui::Ui, _world: &mut World) {
    ui.label("MMB: Move camera");
    ui.label("Shift + MMB: Rotate camera");
}

fn selection_ui(ui: &mut egui::Ui, world: &mut World) {
    let mut selection = world.query_filtered::<Entity, WithSelected>();

    let num_selected = selection.iter(world).count();
    let selection_text = match num_selected {
        0 => "Nothing selected".to_string(),
        1 => "1 entity selected".to_string(),
        n => format!("{n} entities selected"),
    };

    ui.label(selection_text);

    if num_selected == 0 {
        return;
    }

    ui.label("Esc: cancel selection");

    xform_ops_ui(ui, world);
}

fn xform_ops_ui(ui: &mut egui::Ui, world: &mut World) {
    let op = world.resource::<TransformOp>();

    ui.label(format!("{}", *op));

    match op {
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
}
