//! Egui Tiles
//!
//!

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_tiles::{SimplificationOptions, Tree};

#[derive(Debug, Resource)]
pub struct UiTilingPane {
    pub id: String,
}

#[derive(Debug, Resource)]
pub struct UiTreeRes(pub Tree<UiTilingPane>);

#[derive(Debug)]
pub struct UiTilingPlugin;

impl Plugin for UiTilingPlugin {
    fn build(&self, app: &mut App) {
        let mut tiles = egui_tiles::Tiles::default();
        let root = tiles.insert_tab_tile(vec![]);

        app.insert_resource(UiTreeRes(Tree::new("my_tree", root, tiles)));
        app.add_systems(PreStartup, add_one_tile);
        app.add_systems(Update, draw_ui);
    }
}

struct TreeBehavior {}

impl egui_tiles::Behavior<UiTilingPane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &UiTilingPane) -> egui::WidgetText {
        format!("Pane {}", pane.id).into()
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        pane: &mut UiTilingPane,
    ) -> egui_tiles::UiResponse {
        // Give each pane a unique color:
        let color = egui::epaint::Hsva::new(0.103 * tile_id.0 as f32, 0.5, 0.5, 1.0);
        ui.painter().rect_filled(ui.max_rect(), 0.0, color);

        ui.label(format!("The contents of pane {:?}.", tile_id));

        // You can make your pane draggable like so:
        if ui
            .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }
}

fn draw_ui(mut contexts: EguiContexts, mut r_tree: ResMut<UiTreeRes>) {
    let ctx = contexts.ctx_mut();

    let tree = &mut r_tree.0;

    egui::CentralPanel::default().show(ctx, |ui| {
        let mut behavior = TreeBehavior {};
        tree.ui(&mut behavior, ui);
    });
}

fn add_one_tile(mut tree: ResMut<UiTreeRes>) {
    let pane = UiTilingPane { id: 1.to_string() };

    let child = tree.0.tiles.insert_pane(pane);
    let _tile_id = tree.0.root.insert(child);
    //let horizontal = tree.0.tiles.insert_horizontal_tile(vec![child]);
}

fn create_tree() -> UiTreeRes {
    let mut next_view_nr = 0;
    let mut gen_pane = || {
        let pane = UiTilingPane {
            id: next_view_nr.to_string(),
        };
        next_view_nr += 1;
        pane
    };

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];

    tabs.push({
        let children = (0..7).map(|_| tiles.insert_pane(gen_pane())).collect();
        tiles.insert_horizontal_tile(children)
    });

    let root = tiles.insert_tab_tile(tabs);

    UiTreeRes(Tree::new("my_tree", root, tiles))
}
