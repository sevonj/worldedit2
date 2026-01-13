use bevy::asset::RenderAssetUsages;
use bevy::math::{Vec3A, vec3a};
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;

use crate::editor::Colors;

pub struct GridFloorPlugin;

const GRID_SIZE: i32 = 512;

impl Plugin for GridFloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let level_0 = meshes.add(grid_mesh(1.));
    let level_1 = meshes.add(grid_mesh(10.));
    let x_axis = meshes.add(
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all()).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                vec3a((-GRID_SIZE / 2) as f32, 0., 0.),
                vec3a((GRID_SIZE / 2) as f32, 0., 0.),
            ],
        ),
    );
    let z_axis = meshes.add(
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all()).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                vec3a(0., 0., (-GRID_SIZE / 2) as f32),
                vec3a(0., 0., (GRID_SIZE / 2) as f32),
            ],
        ),
    );

    commands.spawn((
        Mesh3d(level_0),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Colors::GRID_MINOR.into(),
            unlit: true,
            ..default()
        })),
    ));
    commands.spawn((
        Mesh3d(level_1),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Colors::GRID_MAJOR.into(),
            unlit: true,
            ..default()
        })),
    ));
    commands.spawn((
        Mesh3d(x_axis),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Colors::AXIS_X.into(),
            unlit: true,
            ..default()
        })),
    ));
    commands.spawn((
        Mesh3d(z_axis),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Colors::AXIS_Z.into(),
            unlit: true,
            ..default()
        })),
    ));
}

//#[derive(Component, Debug)]
//struct GridFloor;

/// Gets you both X and Z lines for a given offset
fn line_vertices(size: f32, offset: f32) -> Vec<Vec3A> {
    vec![
        vec3a(-size / 2., 0., offset),
        vec3a(size / 2., 0., offset),
        vec3a(offset, 0., -size / 2.),
        vec3a(offset, 0., size / 2.),
    ]
}

fn grid_mesh(cell_size: f32) -> Mesh {
    let mut vertices = vec![];
    for i in -GRID_SIZE / 2..=GRID_SIZE / 2 {
        if i % 10 == 0 {
            continue;
        }
        vertices.extend(line_vertices(
            GRID_SIZE as f32 * cell_size,
            i as f32 * cell_size,
        ))
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh
}
