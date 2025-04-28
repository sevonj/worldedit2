use bevy::{ecs::query::QueryIter, prelude::*};
use bevy_egui::egui::{self, Ui};
use egui_extras::{Column, TableBuilder};

use crate::editor::{selection::Selected, Selectable};

use super::{
    editor_pane::EditorPane,
    ui_tiling::{TileTree, TilingPane},
};

#[derive(Debug)]
pub struct OutlinerPanePlugin;

impl Plugin for OutlinerPanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_pane);
    }
}

type SelectableQuery<'a> = (Entity, Option<&'a Name>, Option<&'a Selected>);

#[derive(Debug)]
pub struct OutlinerPane;

impl EditorPane for OutlinerPane {
    fn ui(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        world: &mut World,
        commands: &mut Commands,
    ) -> egui_tiles::UiResponse {
        let mut query = world.query_filtered::<SelectableQuery, With<Selectable>>();

        let entities = query.iter(world);
        outliner_ui(ui, commands, entities);

        egui_tiles::UiResponse::None
    }

    fn title(&self) -> String {
        "Outliner".into()
    }
}

fn register_pane(mut tree: ResMut<TileTree>) {
    let tile_id = tree.register_pane(TilingPane::Outliner(OutlinerPane));
    tree.set_share(tile_id, 0.2);
}

fn outliner_ui(
    ui: &mut Ui,
    commands: &mut Commands,
    entities: QueryIter<SelectableQuery, With<Selectable>>,
) {
    let tablebuilder = TableBuilder::new(ui).column(Column::auto());

    tablebuilder.body(|mut body| {
        for (entity, name, selected) in entities {
            let name_str = name.map_or("NO_NAME", Name::as_str);
            let text = format!("{name_str} [{}]", entity);
            let is_selected = selected.is_some();

            body.row(16.0, |mut row| {
                row.col(|ui| {
                    let button = egui::SelectableLabel::new(is_selected, text);
                    if ui.add(button).clicked() {
                        if is_selected {
                            commands.entity(entity).remove::<Selected>();
                        } else {
                            commands.entity(entity).insert(Selected);
                        }
                    };
                });
            });
        }
    });
}
