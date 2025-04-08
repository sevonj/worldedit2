//! Egui Tiles
//!
//!

use bevy::prelude::*;

use super::{editor_pane::EditorPane, outliner_pane::OutlinerPane, viewport_pane::ViewportPane};
use bevy_egui::{
    egui::{self, CentralPanel, Frame, Ui},
    EguiContext, EguiContextPass,
};

use egui_tiles::{Behavior, SimplificationOptions, TileId, Tiles, Tree};

#[derive(Debug, Resource)]
pub enum TilingPane {
    ViewPort(ViewportPane),
    Outliner(OutlinerPane),
}

#[derive(Debug, Resource)]
pub struct TileTree(pub Tree<TilingPane>);

impl TileTree {
    pub fn register_pane(&mut self, pane: TilingPane) {
        let child = self.0.tiles.insert_pane(pane);
        let root = self.0.root.unwrap();
        self.0.move_tile_to_container(child, root, usize::MAX, true);
    }
}

#[derive(Debug)]
pub struct UiTilingPlugin;

impl Plugin for UiTilingPlugin {
    fn build(&self, app: &mut App) {
        let mut tiles = Tiles::default();
        let root = tiles.insert_horizontal_tile(vec![]);

        app.insert_resource(TileTree(Tree::new("my_tree", root, tiles)));
        app.add_systems(EguiContextPass, draw_editor_ui);
    }
}

struct TreeBehavior<'a> {
    pub world: &'a mut World,
}

impl Behavior<TilingPane> for TreeBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &TilingPane) -> egui::WidgetText {
        match pane {
            TilingPane::ViewPort(pane) => pane.title().into(),
            TilingPane::Outliner(pane) => pane.title().into(),
        }
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut Ui,
        _tile_id: TileId,
        pane: &mut TilingPane,
    ) -> egui_tiles::UiResponse {
        match pane {
            TilingPane::ViewPort(pane) => pane.ui(ui, self.world),
            TilingPane::Outliner(pane) => pane.ui(ui, self.world),
        }
    }
}

fn draw_editor_ui(world: &mut World) {
    let Ok(mut context) = world
        .query::<&mut EguiContext>()
        .single_mut(world)
        .map(|w| w.clone())
    else {
        return;
    };
    let ctx = context.get_mut();

    world.resource_scope::<TileTree, _>(|world, mut tree| {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            let mut behavior = TreeBehavior { world };
            tree.0.ui(&mut behavior, ui);
        });
    });
}
