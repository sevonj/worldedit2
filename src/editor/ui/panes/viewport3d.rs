use bevy::prelude::*;

use bevy_egui::EguiContexts;
use bevy_egui::EguiUserTextures;
use bevy_egui::egui;
use bevy_egui::egui::Frame;

use super::EditorPane;
use crate::editor::resources::ViewportRect;
use crate::editor::resources::ViewportRenderTarget;
use crate::editor::selection::WithSelected;
use crate::editor::selection_actions::transform_action::TransformAction;
use crate::editor::ui::ui_tiling::TileTree;
use crate::editor::ui::ui_tiling::TilingPane;

#[derive(Debug)]
pub struct ViewportPanePlugin;

impl Plugin for ViewportPanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, create_viewport_img);
        app.add_systems(Startup, register_pane);
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

    fn tab_title(&self) -> &'static str {
        "Scene"
    }
}

pub fn create_viewport_img(
    egui_user_textures: ResMut<EguiUserTextures>,
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
) {
    commands.insert_resource(ViewportRenderTarget::new(egui_user_textures, images));
}

fn register_pane(
    mut tree: ResMut<TileTree>,
    contexts: EguiContexts,
    viewport_img: Res<ViewportRenderTarget>,
    mut commands: Commands,
) {
    let viewport_tex_id = contexts.image_id(&**viewport_img).unwrap();
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
    let op = world.resource::<TransformAction>();

    ui.label(format!("{}", *op));

    match op {
        TransformAction::None => {
            ui.label("G: Move");
            ui.label("R: Rotate");
            ui.label("S: Scale");
        }
        TransformAction::Move { axis_lock, .. }
        | TransformAction::Rotate { axis_lock, .. }
        | TransformAction::Scale(axis_lock) => {
            ui.label("Esc: cancel selection");
            ui.label(format!("axis: {axis_lock}"));
        }
    }
}
