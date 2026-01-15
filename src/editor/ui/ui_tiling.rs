//! Egui Tiles
//!
//!

use bevy::prelude::*;

use bevy::ecs::world::CommandQueue;
use bevy_egui::EguiContext;
use bevy_egui::EguiPrimaryContextPass;
use bevy_egui::egui;
use bevy_egui::egui::CentralPanel;
use bevy_egui::egui::Frame;
use bevy_egui::egui::Ui;
use egui_tiles::{Behavior, Container, SimplificationOptions, Tile, TileId, Tiles, Tree};

use super::panes::EditorPane;

use super::panes::OutlinerPane;
use super::panes::ViewportPane;

#[derive(Debug, Resource)]
pub enum TilingPane {
    ViewPort(ViewportPane),
    Outliner(OutlinerPane),
}

#[derive(Debug, Resource)]
pub struct TileTree(pub Tree<TilingPane>);

impl TileTree {
    pub fn register_pane(&mut self, pane: TilingPane) -> TileId {
        let child = self.0.tiles.insert_pane(pane);
        let root = self.0.root.unwrap();
        self.0.move_tile_to_container(child, root, usize::MAX, true);
        child
    }

    pub fn set_share(&mut self, tile_id: TileId, share: f32) {
        let Some(root) = self.0.root() else {
            return;
        };
        let Some(Tile::Container(Container::Linear(linear))) = self.0.tiles.get_mut(root) else {
            return;
        };
        linear.shares.set_share(tile_id, share);
    }
}

#[derive(Debug)]
pub struct UiTilingPlugin;

impl Plugin for UiTilingPlugin {
    fn build(&self, app: &mut App) {
        let mut tiles = Tiles::default();
        let root = tiles.insert_horizontal_tile(vec![]);

        app.insert_resource(TileTree(Tree::new("my_tree", root, tiles)));
        app.add_systems(EguiPrimaryContextPass, draw_editor_ui);
    }
}

struct TreeBehavior<'a> {
    pub world: &'a mut World,
    pub commands: Commands<'a, 'a>,
}

impl Behavior<TilingPane> for TreeBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &TilingPane) -> egui::WidgetText {
        match pane {
            TilingPane::ViewPort(pane) => pane.tab_title().into(),
            TilingPane::Outliner(pane) => pane.tab_title().into(),
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
            TilingPane::ViewPort(pane) => pane.ui(ui, self.world, &mut self.commands),
            TilingPane::Outliner(pane) => pane.ui(ui, self.world, &mut self.commands),
        }
    }
}

fn draw_editor_ui(world: &mut World) {
    // Extract egui context from the world
    let Ok(mut context) = world
        .query::<&mut EguiContext>()
        .single_mut(world)
        .map(|w| w.clone())
    else {
        return;
    };
    let ctx = context.get_mut();

    // Extract commands from the world
    // ⚠️: unsafe, unclear how dangerous this is.
    let unsafe_world_cell = world.as_unsafe_world_cell();
    let mut queue = CommandQueue::default();
    let commands = Commands::new(&mut queue, unsafe { unsafe_world_cell.world() });
    let world = unsafe { unsafe_world_cell.world_mut() };

    world.resource_scope::<TileTree, _>(|world, mut tree| {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            let mut behavior = TreeBehavior { world, commands };
            tree.0.ui(&mut behavior, ui);
        });
    });

    queue.apply(world);
}
