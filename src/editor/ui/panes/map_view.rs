use bevy::prelude::*;

use bevy_egui::EguiContexts;
use bevy_egui::egui;
use bevy_egui::egui::Frame;

use super::EditorPane;
use crate::editor::camera_rig_orbital::CameraRigOrbital;
use crate::editor::components::ViewportRect;
use crate::editor::components::ViewportRenderTarget;
use crate::editor::selection::WithSelected;
use crate::editor::selection_actions::transform_action::TransformAction;
use crate::editor::ui::ui_tiling::TileTree;
use crate::editor::ui::ui_tiling::TilingPane;

#[derive(Component)]
struct BelongsToMapView;

#[derive(Debug)]
pub struct MapViewPanePlugin;

impl Plugin for MapViewPanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, MapViewPane::create);
    }
}

#[derive(Debug)]
pub struct MapViewPane {
    rt_texture_id: egui::TextureId,
}

impl MapViewPane {
    fn create(
        mut contexts: EguiContexts,
        mut tree: ResMut<TileTree>,
        images: ResMut<Assets<Image>>,
        mut commands: Commands,
    ) {
        let render_target = ViewportRenderTarget::new(&mut contexts, images);
        let rt_texture_id = contexts.image_id(&render_target.img).unwrap();
        CameraRigOrbital::spawn_with_name(&mut commands, "MapView Camera").insert((
            render_target,
            ViewportRect::default(),
            BelongsToMapView,
        ));
        tree.register_pane(TilingPane::MapView(Self { rt_texture_id }));
    }
}

impl EditorPane for MapViewPane {
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
                    self.rt_texture_id,
                    rect.size(),
                ));
                image.paint_at(ui, rect);

                camera_controls_ui(ui, world);

                if let Ok(mut rect_res) = world
                    .query_filtered::<&mut ViewportRect, With<BelongsToMapView>>()
                    .single_mut(world)
                {
                    *rect_res = ViewportRect::from(rect);
                }
            });

        egui_tiles::UiResponse::None
    }

    fn tab_title(&self) -> &'static str {
        "Map View"
    }
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
