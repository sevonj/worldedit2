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

/// Store area occupied by viewport
#[derive(Debug, Resource, Clone, Copy)]
pub struct ViewportRect {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Default for ViewportRect {
    fn default() -> Self {
        // Non-zero placeholder size
        Self {
            min_x: 16.,
            min_y: 16.,
            max_x: 16.,
            max_y: 16.,
        }
    }
}

impl From<egui::Rect> for ViewportRect {
    fn from(rect: egui::Rect) -> Self {
        Self {
            min_x: rect.min.x,
            min_y: rect.min.y,
            max_x: rect.max.x,
            max_y: rect.max.y,
        }
    }
}

impl ViewportRect {
    pub const fn size(&self) -> egui::Vec2 {
        egui::Vec2 {
            x: self.max_x - self.min_x,
            y: self.max_y - self.min_y,
        }
    }
}

#[derive(Debug)]
pub struct ViewportPane {
    viewport_tex_id: egui::TextureId,
}

impl ViewportPane {
    pub fn new(viewport_tex_id: egui::TextureId) -> Self {
        Self { viewport_tex_id }
    }
}

impl EditorPane for ViewportPane {
    fn ui(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        world: &mut World,
        _commands: &mut Commands,
    ) -> egui_tiles::UiResponse {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show_inside(ui, |ui| {
                egui::TopBottomPanel::bottom("viewport_bottom").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        selection_ui(ui, world);
                    });
                });

                let rect = ui.available_rect_before_wrap();

                let image = egui::Image::new(egui::load::SizedTexture::new(
                    self.viewport_tex_id,
                    rect.size(),
                ));
                image.paint_at(ui, rect);

                camera_controls_ui(ui, world);

                let mut rect_res = world.resource_mut::<ViewportRect>();
                *rect_res = ViewportRect::from(rect);
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
    mut commands: Commands,
) {
    let viewport_tex_id = contexts.image_id(&viewport_img).unwrap();
    let pane = TilingPane::ViewPort(ViewportPane::new(viewport_tex_id));
    tree.register_pane(pane);

    commands.insert_resource(ViewportRect::default());
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
