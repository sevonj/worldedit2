use std::path::PathBuf;

use bevy::prelude::*;

use bevy::pbr::wireframe::Wireframe;

use worldedit::terrain_processing::TerrainMesh;
use worldedit::terrain_processing::terrain_cruncher;

use worldedit::terrain_processing::CACHE_DIR;
use worldedit::terrain_processing::NUM_CELLS;
use worldedit::terrain_processing::WORLD_SIZE;

pub struct TerrainCellPreviewPlugin;

impl Plugin for TerrainCellPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    terrain_cruncher::crunch_terrain();

    for i in 0..NUM_CELLS {
        let cell_name = format!("cell_{i:03}");
        let cell_path = PathBuf::from(CACHE_DIR)
            .join(&cell_name)
            .with_extension(TerrainMesh::FILE_EXT);
        let cell = TerrainMesh::load(&cell_path).unwrap();

        let mesh = meshes.add(cell.bevy_mesh());
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                //base_color_texture: Some(cell_diff),
                //unlit: true,
                ..default()
            })),
            Transform::default().with_translation(vec3(
                -(WORLD_SIZE as f32 / 2.0),
                0.,
                -(WORLD_SIZE as f32 / 2.0),
            )),
            Wireframe,
        ));
    }
}
