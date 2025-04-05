use bevy::prelude::*;
use bevy_egui::{
    egui::{self, SidePanel, Ui},
    EguiContexts,
};
use egui_extras::{Column, TableBuilder};

use crate::{editor::selection::Selected, spline::Spline};

#[derive(Debug)]
pub struct MainUi;

impl Plugin for MainUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_ui);
    }
}

type SelectableQuery<'a> = (Entity, Option<&'a Name>, Option<&'a Selected>);

fn draw_ui(
    mut contexts: EguiContexts,
    commands: Commands,
    entities: Query<SelectableQuery, With<Spline>>,
    test_entities: Query<TestQuery>,
) {
    let ctx = contexts.ctx_mut();

    SidePanel::right("panel_righ").show(ctx, |ui| {
        spline_list(ui, commands, entities);

        ui.separator();

        xform_list(ui, test_entities);
    });
}

fn spline_list(
    ui: &mut Ui,
    mut commands: Commands,
    entities: Query<SelectableQuery, With<Spline>>,
) {
    let tablebuilder = TableBuilder::new(ui)
        .column(Column::auto())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Splines");
            });
        });

    tablebuilder.body(|mut body| {
        for (entity, name, selected) in entities.iter() {
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
pub type TestQuery<'a> = (
    Option<Entity>,
    Option<&'a Transform>,
    Option<&'a Name>,
    Option<&'a Selected>,
);

fn xform_list(ui: &mut Ui, entities: Query<TestQuery>) {
    let tablebuilder = TableBuilder::new(ui)
        .id_salt("test_table")
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("All entities");
            });
            header.col(|ui| {
                ui.heading("xform");
            });
            header.col(|ui| {
                ui.heading("selected");
            });
        });

    tablebuilder.body(|mut body| {
        for (entity, transform, name, selected) in entities.iter() {
            let name_str = name.map_or("NO_NAME", Name::as_str);
            let text = format!("{name_str} [{:?}]", entity);
            let is_selected = selected.is_some();

            body.row(16.0, |mut row| {
                row.col(|ui| {
                    let button = egui::SelectableLabel::new(is_selected, text);
                    ui.add(button);
                });
                row.col(|ui| {
                    ui.label(format!("{transform:?}"));
                });
                row.col(|ui| {
                    ui.label(format!("{:?}", selected.is_some()));
                });
            });
        }
    });
}
