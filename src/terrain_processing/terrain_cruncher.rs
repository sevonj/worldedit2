use std::path::PathBuf;

use bevy::math::prelude::*;
use image::ImageReader;

use crate::terrain_processing::CACHE_DIR;
use crate::terrain_processing::CELL_SIZE;
use crate::terrain_processing::HeightmapBundle;
use crate::terrain_processing::NUM_CELLS;
use crate::terrain_processing::NUM_CELLS_ROW;
use crate::terrain_processing::TerrainMesh;
use crate::terrain_processing::heightmap;

pub fn crunch_terrain() {
    if std::fs::exists(CACHE_DIR).unwrap() {
        std::fs::remove_dir_all(CACHE_DIR).unwrap();
    }
    std::fs::create_dir_all(CACHE_DIR).unwrap();

    let base_map = heightmap::from_dynamic_image(
        ImageReader::open("assets/heightmaps/test_island_0.png")
            .unwrap()
            .decode()
            .unwrap(),
    );
    heightmap::save(
        &PathBuf::from(CACHE_DIR).join("base_heightmap.hmp"),
        &base_map,
    )
    .unwrap();
    heightmap::save_png(
        &PathBuf::from(CACHE_DIR).join("base_heightmap.png"),
        &base_map,
    )
    .unwrap();
    let h_bundle = HeightmapBundle::new(base_map);

    for i in 0..NUM_CELLS {
        let cell_name = format!("cell_{i:03}");
        let cell_position = uvec2(
            ((i % NUM_CELLS_ROW) * CELL_SIZE) as u32,
            ((i / NUM_CELLS_ROW) * CELL_SIZE) as u32,
        );
        let cell_mesh_path = PathBuf::from(CACHE_DIR)
            .join(&cell_name)
            .with_extension(TerrainMesh::FILE_EXT);

        let cell = TerrainMesh::new(cell_position, &|c| h_bundle.height(c));
        cell.save(&cell_mesh_path).unwrap();
    }
}
